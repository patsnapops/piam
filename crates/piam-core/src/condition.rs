pub mod input {
    use std::net::SocketAddr;

    use serde::{Deserialize, Serialize};

    use crate::input::Input;

    #[derive(Clone, Debug, Default)]
    pub struct ConditionCtx {
        pub from: Condition,
        pub proxy: Condition,
        pub to: Condition,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Condition {
        pub addr: Option<SocketAddr>,
        pub region: Option<String>,
        pub env: Option<String>,
    }

    impl ConditionCtx {
        pub fn from(mut self, from: Condition) -> Self {
            self.from = from;
            self
        }

        pub fn proxy(mut self, proxy: Condition) -> Self {
            self.proxy = proxy;
            self
        }

        pub fn to(mut self, to: Condition) -> Self {
            self.to = to;
            self
        }
    }

    impl Condition {
        pub fn new_with_region_env(region: &str, env: &str) -> Self {
            Self {
                region: Some(region.to_string()),
                env: Some(env.to_string()),
                ..Default::default()
            }
        }

        pub fn new_with_addr(addr: SocketAddr) -> Self {
            Self {
                addr: Some(addr),
                ..Default::default()
            }
        }
    }

    impl Input for ConditionCtx {}
}
