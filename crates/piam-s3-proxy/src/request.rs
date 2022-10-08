use http::{header::HOST, uri::PathAndQuery, HeaderValue, Uri};
use piam_core::type_alias::HttpRequest;

use crate::config::S3_CONFIG;

pub trait S3RequestTransform {
    /// convert path-style-url to virtual hosted style
    /// https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-bucket-intro.html
    fn adapt_path_style(&mut self, path: String);

    fn set_actual_host(&mut self);
}

impl S3RequestTransform for HttpRequest {
    fn adapt_path_style(&mut self, path: String) {
        let proxy_host = &S3_CONFIG.load().proxy_host;
        if self.headers().get(HOST).unwrap().to_str().unwrap() == proxy_host {
            // get content of path before first '/'
            let bucket = path.split('/').next().unwrap();

            // remove bucket from uri
            let mut uri_without_bucket = self
                .uri_mut()
                .path_and_query()
                .unwrap()
                .as_str()
                .replace(format!("/{}", bucket).as_str(), "");
            if uri_without_bucket.is_empty() {
                uri_without_bucket = "/".to_string();
            }
            *self.uri_mut() = Uri::builder()
                .path_and_query(PathAndQuery::try_from(uri_without_bucket).unwrap())
                .build()
                .unwrap();

            // add bucket to host
            self.headers_mut().insert(
                HOST,
                HeaderValue::from_str(format!("{}.{}", bucket, proxy_host).as_str()).unwrap(),
            );
        }
    }

    fn set_actual_host(&mut self) {
        let config = S3_CONFIG.load();
        let host = self
            .headers()
            .get(HOST)
            .unwrap()
            .to_str()
            .unwrap()
            .replace(&config.proxy_host, &config.actual_host);
        self.headers_mut()
            .insert(HOST, HeaderValue::from_str(host.as_str()).unwrap());

        let uri = format!("http://{}{}", config.actual_host, self.uri());
        *self.uri_mut() = Uri::try_from(uri).unwrap();
    }
}
