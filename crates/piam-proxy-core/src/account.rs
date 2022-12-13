//! Account is an abstraction of cloud accounts such as AWS account, GCP Account, Tencent Cloud Account, etc.

use crate::type_alias::IamEntityIdType;

pub type AccountId = IamEntityIdType;

pub mod aws {
    use serde::{Deserialize, Serialize};

    use crate::account::AccountId;

    /// currently only aws sigv4 compatible
    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
    pub struct AwsAccount {
        pub id: AccountId,
        pub code: String,
        pub access_key: String,
        pub secret_key: String,
        pub comment: String,
    }

    impl std::fmt::Display for AwsAccount {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "AWS Account: id: {} code: {} access_key: {}",
                self.id, self.code, self.access_key
            )
        }
    }
}
