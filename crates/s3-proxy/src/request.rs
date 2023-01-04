use http::{header::HOST, uri::PathAndQuery, HeaderValue, Uri};
use piam_proxy::{
    error::{ProxyError, ProxyResult},
    request::from_region_to_host,
    type_alias::HttpRequest,
};

use crate::{error::from_parser_into_proxy_error, S3Config};

pub trait S3RequestTransform {
    /// convert path-style-url to virtual hosted style
    /// <https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-bucket-intro.html>
    fn adapt_path_style(&mut self, path: String, proxy_hosts: &[String]) -> ProxyResult<()>;

    fn set_actual_host(&mut self, config: &S3Config, actual_host: &str) -> ProxyResult<()>;
}

impl S3RequestTransform for HttpRequest {
    fn adapt_path_style(&mut self, path: String, proxy_hosts: &[String]) -> ProxyResult<()> {
        let host = self
            .headers()
            .get(HOST)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        if proxy_hosts.contains(&host) {
            // get content of path before first '/'
            let bucket = path.split('/').next().ok_or_else(|| {
                ProxyError::MalformedProtocol(format!("path should start with /, but got {}", path))
            })?;

            // remove bucket from uri
            let mut uri_without_bucket = self
                .uri_mut()
                .path_and_query()
                .ok_or_else(|| {
                    ProxyError::MalformedProtocol("path_and_query should not be None".to_string())
                })?
                .as_str()
                .strip_prefix(&format!("/{}", bucket))
                .ok_or_else(|| {
                    ProxyError::MalformedProtocol(format!(
                        "path_and_query should start with /{}",
                        bucket
                    ))
                })?;
            if uri_without_bucket.is_empty() {
                uri_without_bucket = "/";
            }
            *self.uri_mut() = Uri::builder()
                .path_and_query(PathAndQuery::try_from(uri_without_bucket).unwrap())
                .build()
                .unwrap();

            // add bucket to host
            self.headers_mut().insert(
                HOST,
                HeaderValue::from_str(format!("{}.{}", bucket, host).as_str()).unwrap(),
            );
        }
        Ok(())
    }

    fn set_actual_host(&mut self, config: &S3Config, region: &str) -> ProxyResult<()> {
        let host = self.headers().get(HOST).unwrap().to_str().unwrap();
        let proxy_host = config
            .proxy_hosts
            .find_proxy_host(host)
            .map_err(from_parser_into_proxy_error)?;
        let bucket_dot = host.strip_suffix(proxy_host).ok_or_else(|| {
            ProxyError::InvalidEndpoint(format!("host {} should end with {}", host, proxy_host))
        })?;
        let actual_host = from_region_to_host(region)?;
        let host = format!("{}{}", bucket_dot, actual_host);
        self.headers_mut()
            .insert(HOST, HeaderValue::from_str(&host).unwrap());

        let uri = format!("http://{}{}", actual_host, self.uri());
        *self.uri_mut() = Uri::try_from(uri).unwrap();
        Ok(())
    }
}
