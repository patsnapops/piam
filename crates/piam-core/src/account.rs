//! Account is an abstraction of cloud accounts such as [`aws::AwsAccount`], GCP Account, Tencent Cloud Account, etc.

use crate::type_alias::IamEntityIdType;

pub type AccountId = IamEntityIdType;

pub mod aws {
    use std::fmt::Debug;

    use serde::{Deserialize, Serialize};

    use crate::{account::AccountId, IamIdentity};

    /// currently only aws sigv4 compatible
    #[derive(Clone, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
    pub struct AwsAccount {
        pub id: AccountId,
        pub code: String,
        pub access_key: String,
        pub secret_key: String,
        pub comment: String,
    }

    impl IamIdentity for AwsAccount {
        fn id_str(&self) -> &str {
            &self.id
        }
    }

    impl Debug for AwsAccount {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "AwsAccount {{ id: {}, code: {}, comment: {} }}",
                self.id, self.code, self.comment
            )
        }
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
