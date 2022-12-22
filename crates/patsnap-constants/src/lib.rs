pub mod region {
    use serde::{Deserialize, Serialize};
    use strum_macros::Display;

    pub const ANY: &str = "any";

    pub const CN_NORTHWEST_1: &str = "cn-northwest-1";
    pub const US_EAST_1: &str = "us-east-1";

    pub const AP_SHANGHAI: &str = "ap-shanghai";
    pub const NA_ASHBURN: &str = "na-ashburn";

    pub const SU_ZHOU: &str = "su-zhou";
    pub const LOCAL: &str = "local";

    #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, Display)]
    #[serde(rename_all = "kebab-case")]
    pub enum Region {
        #[default]
        Any,
        #[serde(rename = "cn-northwest-1")]
        CnNorthwest1,
        #[serde(rename = "us-east-1")]
        UsEast1,
        ApShanghai,
        NaAshburn,
        SuZhou,
        Local,
        Unknown,
    }

    impl From<&str> for Region {
        fn from(s: &str) -> Self {
            match s {
                ANY => Self::Any,
                CN_NORTHWEST_1 => Self::CnNorthwest1,
                US_EAST_1 => Self::UsEast1,
                AP_SHANGHAI => Self::ApShanghai,
                NA_ASHBURN => Self::NaAshburn,
                SU_ZHOU => Self::SuZhou,
                LOCAL => Self::Local,
                _ => Self::Unknown,
            }
        }
    }

    impl From<Region> for String {
        fn from(r: Region) -> Self {
            match r {
                Region::Any => ANY.to_string(),
                Region::CnNorthwest1 => CN_NORTHWEST_1.to_string(),
                Region::UsEast1 => US_EAST_1.to_string(),
                Region::ApShanghai => AP_SHANGHAI.to_string(),
                Region::NaAshburn => NA_ASHBURN.to_string(),
                Region::SuZhou => SU_ZHOU.to_string(),
                Region::Local => LOCAL.to_string(),
                Region::Unknown => "unknown".to_string(),
            }
        }
    }
}

pub mod env {
    use serde::{Deserialize, Serialize};
    use strum_macros::Display;

    pub const ANY: &str = "any";

    pub const CI: &str = "ci";
    pub const QA: &str = "qa";
    pub const RELEASE: &str = "release";
    pub const PROD: &str = "prod";

    #[derive(Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, Display)]
    #[serde(rename_all = "kebab-case")]
    pub enum Env {
        #[default]
        Any,
        Ci,
        Qa,
        Release,
        Prod,
    }
}

pub mod account {
    use serde::{Deserialize, Serialize};
    use strum_macros::Display;

    pub const ANY: &str = "any";

    pub const AWS_DEV_9554: &str = "cn_aws_dev_9554";
    pub const AWS_PROD_3977: &str = "cn_aws_prod_3977";
    pub const AWS_PROD_7478: &str = "us_aws_prod_7478";
    pub const AWS_DATA_0066: &str = "us_aws_data_0066";
    pub const TENCENT_4258: &str = "cn_tencent_4258";

    #[derive(Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, Display)]
    pub enum Account {
        #[default]
        Any,
        #[serde(rename = "cn_aws_dev_9554")]
        AwsDev9554,
        #[serde(rename = "cn_aws_prod_3977")]
        AwsProd3977,
        #[serde(rename = "us_aws_prod_7478")]
        AwsProd7478,
        #[serde(rename = "us_aws_data_0066")]
        AwsData0066,
        #[serde(rename = "cn_tencent_4258")]
        Tencent4258,
    }
}

pub mod policy_model {
    pub const ANY: &str = "any";
    pub const OBJECT_STORAGE: &str = "ObjectStorage";
}

pub mod s3_proxy_endpoint {
    pub const EP_INTERNAL: &str = "http://internal.s3-proxy.patsnap.info";
    pub const EP_US_EAST_1: &str = "http://us-east-1.s3-proxy.patsnap.info";
    pub const EP_CN_NORTHWEST_1: &str = "http://cn-northwest-1.s3-proxy.patsnap.info";
    pub const EP_NA_ASHBURN: &str = "http://na-ashburn.s3-proxy.patsnap.info";
    pub const EP_AP_SHANGHAI: &str = "http://ap-shanghai.s3-proxy.patsnap.info";
    pub const EP_LOCAL: &str = "http://local.s3-proxy.patsnap.info";
    pub const EP_S3_PROXY_DEV: &str = "http://s3-proxy.dev";

    pub const EPS_NON_DEV: &[&str] = &[
        EP_US_EAST_1,
        EP_CN_NORTHWEST_1,
        EP_NA_ASHBURN,
        EP_AP_SHANGHAI,
        EP_LOCAL,
    ];
}

pub const ANY: &str = "any";

pub const IP_PROVIDER: &str = "http://cip.cc";
