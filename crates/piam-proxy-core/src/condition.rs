pub mod input {
    use std::net::SocketAddr;

    use serde::{Deserialize, Serialize};

    use crate::input::Input;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Condition {
        pub addr: Option<SocketAddr>,
        pub from_region: Option<String>,
    }

    impl Condition {
        pub fn new_with_addr(addr: SocketAddr) -> Self {
            Self {
                addr: Some(addr),
                from_region: None,
            }
        }
    }

    impl Input for Condition {}
}
