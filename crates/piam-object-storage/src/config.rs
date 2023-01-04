use serde::{Deserialize, Serialize};

use crate::error::{ParserError, ParserResult};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HostDomains {
    pub domains: Vec<String>,
}

impl HostDomains {
    pub fn find_proxy_host(&self, host: &str) -> ParserResult<&str> {
        let s = self
            .domains
            .iter()
            .find(|&v| host.ends_with(v))
            .ok_or_else(|| {
                ParserError::InvalidEndpoint(format!(
                    "'{host}' is not ending with a valid piam s3 proxy endpoint"
                ))
            })?;
        Ok(s)
    }
}
