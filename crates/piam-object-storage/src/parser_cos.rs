use piam_core::type_alias::HttpRequest;

use crate::{config::HostDomains, error::ParserResult, input::ObjectStorageInput};

impl ObjectStorageInput {
    #[allow(unused)] // !!! remove this line when complete
    pub fn from_cos(req: &HttpRequest, config: &HostDomains) -> ParserResult<Self> {
        todo!("convert cos request to ObjectStorageInput")
    }
}

#[cfg(test)]
pub mod test {
    use piam_core::type_alias::HttpRequest;

    use crate::{config::HostDomains, input::ObjectStorageInput};

    #[test]
    fn test_from_cos() {
        let expect = ObjectStorageInput::GetObject {
            bucket: "foo".to_string(),
            key: "bar".to_string(),
        };

        let http_request = {
            // TODO: make a cos GetObject request
            HttpRequest::default()
        };
        let host_domains = HostDomains {
            // TODO: make host domains that parser need
            domains: vec![],
        };
        let actual = ObjectStorageInput::from_cos(&http_request, &host_domains).unwrap();

        // TODO: make the test pass
        assert_eq!(expect, actual);
    }
}
