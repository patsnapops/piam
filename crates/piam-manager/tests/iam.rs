#![allow(unused)]
#![allow(clippy::all)]

use busylib::ANY;
use patsnap_constants::{
    account::{AWS_DATA_0066, AWS_DEV_9554, AWS_PROD_3977, AWS_PROD_7478, TENCENT_4258},
    env::Env::Prod,
    key::AKPSSVCS07PIAMDEV,
    policy_model::OBJECT_STORAGE,
    region::{
        Region,
        Region::{ApShanghai, CnNorthwest1, Local, NaAshburn, UsEast1},
        LOCAL,
    },
};
use piam_core::{
    account::aws::AwsAccount,
    effect::Effect,
    group::Group,
    manager_api_constant::{CONDITION, VERSION},
    policy::{
        condition::{private_ip_cidr, ConditionPolicy, ConditionRange, Range},
        Name, Policy,
    },
    principal::{User, UserKind},
    relation_model::{PolicyRelationship, UserGroupRelationship},
};
use piam_object_storage::{
    parser_s3::S3HostDomains,
    policy::{Bucket, Key, ObjectStorageInputPolicy, ObjectStoragePolicy},
};
use redis::{Client, Commands};
use serde::{ser, Deserialize, Serialize};
use uuid::Uuid;

pub fn make_accounts() -> Vec<AwsAccount> {
    let account_cn_aws_dev_9554 = AwsAccount {
        id: AWS_DEV_9554.into(),
        code: "9554".into(),
        access_key: "AKIA545RXJQZANGAE744".into(),
        secret_key: "".into(),
        comment: "piam cn_aws_dev_9554".to_string(),
    };
    let account_cn_aws_prod_3977 = AwsAccount {
        id: AWS_PROD_3977.into(),
        code: "3977".into(),
        access_key: "AKIAVZG6PPVKGB77FSFI".into(),
        secret_key: "".into(),
        comment: "".to_string(),
    };
    let account_us_aws_prod_7478 = AwsAccount {
        id: AWS_PROD_7478.into(),
        code: "7478".into(),
        access_key: "AKIA24IGUMII4I3EGYOU".into(),
        secret_key: "".into(),
        comment: "".to_string(),
    };
    let account_us_aws_data_0066 = AwsAccount {
        id: AWS_DATA_0066.into(),
        code: "0066".into(),
        access_key: "AKIAQDDYEQIRTBKNVKFR".into(),
        secret_key: "".into(),
        comment: "".to_string(),
    };
    let account_cn_tencent_4258 = AwsAccount {
        id: TENCENT_4258.to_string(),
        code: "4258".to_string(),
        access_key: "AKIDlT7kM0dGqOwS1Y4b7fjFkDdCospljYFm".to_string(),
        secret_key: "".to_string(),
        comment: "Temporarily Solution! this account contains dev/release/prod resources"
            .to_string(),
    };
    // TODO: refactor this quick and dirty solution for s3 uni-key feature
    let account_us_tencent_4258 = AwsAccount {
        id: "us_tencent_4258".to_string(),
        code: "4258".to_string(),
        access_key: "AKIDlT7kM0dGqOwS1Y4b7fjFkDdCospljYFm".to_string(),
        secret_key: "".to_string(),
        comment: "Temporarily Solution! this account contains dev/release/prod resources"
            .to_string(),
    };
    vec![
        account_cn_aws_dev_9554,
        account_cn_aws_prod_3977,
        account_us_aws_prod_7478,
        account_us_aws_data_0066,
        account_cn_tencent_4258,
        account_us_tencent_4258,
    ]
}

pub fn user_3_cjj0() -> User {
    User {
        id: "c43e349a-0860-446e-9d2c-5bbc4211df79".to_string(),
        name: "曹金娟".to_string(),
        base_access_key: "AKPSPERS03CJJ0Z".to_string(),
        secret: "".to_string(),
        kind: Default::default(),
    }
}

pub fn user_3_shf0() -> User {
    User {
        id: "56462cb1-4b8b-4a8a-97a2-ac2f5d2c714f".to_string(),
        name: "宋海峰".to_string(),
        base_access_key: "AKPSPERS03SHF0Z".to_string(),
        secret: "".to_string(),
        kind: Default::default(),
    }
}

pub fn user_3_qwt0() -> User {
    User {
        id: "77cb60d9-2498-4ecb-840d-bf4ab130a584".to_string(),
        name: "钱伟涛".to_string(),
        base_access_key: "AKPSPERS03QWT0Z".to_string(),
        secret: "".to_string(),
        kind: Default::default(),
    }
}

pub fn user_3_fxd0() -> User {
    User {
        id: "63b8d346-d550-496c-bdbe-1b6708295ec4".to_string(),
        name: "方晓东".to_string(),
        base_access_key: "AKPSPERS03FXD0Z".to_string(),
        secret: "".to_string(),
        kind: Default::default(),
    }
}

pub fn user_3_zsz0() -> User {
    User {
        id: "c235b21f-0800-47ff-b4f3-8e77ed183b9d".to_string(),
        name: "张书宗".to_string(),
        base_access_key: "AKPSPERS03ZSZ0Z".to_string(),
        secret: "".to_string(),
        kind: Default::default(),
    }
}

pub fn user_3_whl0() -> User {
    User {
        id: "e1a6a911-9785-49e0-bb81-b25c9914b5d5".to_string(),
        name: "王海龙".to_string(),
        base_access_key: "AKPSPERS03WHL0Z".to_string(),
        secret: "".to_string(),
        kind: Default::default(),
    }
}

pub fn user_3_wwt0() -> User {
    User {
        id: "127BEAD6-78BC-4776-AAF9-07C363E6193C".to_string(),
        name: "吴文涛".to_string(),
        base_access_key: "AKPSPERS03WWT0Z".to_string(),
        secret: "".to_string(),
        kind: Default::default(),
    }
}

pub fn user_3_cyy0() -> User {
    User {
        id: "FDCFC151-D052-4074-BCEC-83ED5D49F3B4".to_string(),
        name: "陈亚运".to_string(),
        base_access_key: "AKPSPERS03CYY0Z".to_string(),
        secret: "".to_string(),
        kind: Default::default(),
    }
}

pub fn user_dev() -> User {
    User {
        id: "28D536B6-35BA-4BC2-9767-7905DEBDFF1E".to_string(),
        name: "user_dev".to_string(),
        base_access_key: AKPSSVCS07PIAMDEV.to_string(),
        secret: "".to_string(),
        kind: UserKind::Service,
    }
}

pub fn user_svcs_d_data_rd_processing_batch_qa() -> User {
    User {
        id: "82b52b79-4130-4846-9779-1ead6f0710dc".to_string(),
        name: "d-data-rd-processing-batch-qa".to_string(),
        base_access_key: "AKPSSVCS24DDATARDPROCESSINGBATCHQA".to_string(),
        secret: "".to_string(),
        kind: UserKind::Service,
    }
}

pub fn user_svcs_d_data_dwc_script() -> User {
    User {
        id: "52ecaf5a-806a-442d-9ee3-5b44490facad".to_string(),
        name: "d-data-dwc-script".to_string(),
        base_access_key: "AKPSSVCS14DDATADWCSCRIPT".to_string(),
        secret: "".to_string(),
        kind: UserKind::Service,
    }
}

pub fn user_svcs_d_data_image_sync_recover() -> User {
    User {
        id: "a0215e6d-88f0-4bd2-a3fe-ba1acf299689".to_string(),
        name: "d-data-image-sync-recover".into(),
        base_access_key: "AKPSSVCS21DDATAIMAGESYNCRECOVER".into(),
        secret: "".to_string(),
        kind: UserKind::Service,
    }
}

pub fn user_svcs_opst() -> User {
    User {
        id: "429fd5dd-fc3f-4237-86e1-6d49c2256506".to_string(),
        name: "user_svcs_opst".into(),
        base_access_key: "AKPSSVCS04OPST".into(),
        secret: "".to_string(),
        kind: UserKind::Service,
    }
}

pub fn user_svcs_data_tmp() -> User {
    User {
        id: "934848a9-76d5-49d0-8165-3676eac0bb58".to_string(),
        name: "user_svcs_data_tmp".into(),
        base_access_key: "AKPSSVCSDATA".into(),
        secret: "".to_string(),
        kind: UserKind::Service,
    }
}

pub fn user_team_data_tmp() -> User {
    User {
        id: "710d54f7-cb5b-436d-b027-4160e4e428a4".to_string(),
        name: "user_team_data_tmp".into(),
        base_access_key: "AKPSTEAMDATA".into(),
        secret: "".to_string(),
        kind: UserKind::Team,
    }
}

pub fn make_users() -> Vec<User> {
    vec![
        user_3_cjj0(),
        user_3_shf0(),
        user_3_qwt0(),
        user_3_fxd0(),
        user_3_zsz0(),
        user_3_whl0(),
        user_3_wwt0(),
        user_3_cyy0(),
        user_dev(),
        user_svcs_d_data_rd_processing_batch_qa(),
        user_svcs_d_data_dwc_script(),
        user_svcs_d_data_image_sync_recover(),
        user_svcs_opst(),
        user_svcs_data_tmp(),
        user_team_data_tmp(),
    ]
}

pub fn group_3_cjj0() -> Group {
    Group {
        id: "8bc9f50f-d3fe-4ff6-950e-e2386ea4f523".to_string(),
        name: "曹金娟".to_string(),
    }
}

pub fn group_3_shf0() -> Group {
    Group {
        id: "226658af-85ca-45cf-918d-77c44469de84".to_string(),
        name: "宋海峰".to_string(),
    }
}

pub fn group_3_qwt0() -> Group {
    Group {
        id: "b0b5b2f5-5b9f-4b9f-9b9c-5b9c5b9c5b9c".to_string(),
        name: "钱伟涛".to_string(),
    }
}

pub fn group_3_fxd0() -> Group {
    Group {
        id: "6d97ae2c-44f0-43c6-8010-143071b34403".to_string(),
        name: "方晓东".to_string(),
    }
}

pub fn group_3_zsz0() -> Group {
    Group {
        id: "b926ae88-2e75-40e1-b63d-6bc4f05b1c01".to_string(),
        name: "张书宗".to_string(),
    }
}

pub fn group_3_whl0() -> Group {
    Group {
        id: "82c50056-9346-43a4-917a-f07d20a6ede9".to_string(),
        name: "王海龙".to_string(),
    }
}

pub fn group_team_sa_dev() -> Group {
    Group {
        id: "59946274-7B3F-4DE8-9BC5-B18F26E0F224".to_string(),
        name: "group_team_sa_dev".to_string(),
    }
}

pub fn group_team_data_dev() -> Group {
    Group {
        id: "5E0E5F1A-1F9C-4F9D-8F5C-1F9C4F9D8F5C".to_string(),
        name: "group_team_data_dev".to_string(),
    }
}

pub fn group_team_data_services() -> Group {
    Group {
        id: "374c9ce4-0fca-4520-93c8-0a74529c07c8".to_string(),
        name: "team-data-services".to_string(),
    }
}

pub fn group_svcs_opst() -> Group {
    Group {
        id: "c0f5b5f5-5b9f-4b9f-9b1f-1b0b0b0b0b0b".to_string(),
        name: "svcs-opst".to_string(),
    }
}

pub fn group_data_tmp() -> Group {
    Group {
        id: "40697633-1d79-4c44-ae50-583595d1dc17".to_string(),
        name: "data-tmp".to_string(),
    }
}

pub fn group_allow_all() -> Group {
    Group {
        id: "EFF02E70-1879-4B10-A861-65313C61FD7D".to_string(),
        name: "allow_all".to_string(),
    }
}

pub fn make_groups() -> Vec<Group> {
    vec![
        group_3_cjj0(),
        group_3_shf0(),
        group_3_qwt0(),
        group_3_fxd0(),
        group_3_zsz0(),
        group_3_whl0(),
        group_team_sa_dev(),
        group_team_data_dev(),
        group_team_data_services(),
        group_svcs_opst(),
        group_data_tmp(),
        group_allow_all(),
    ]
}

fn base_s3_actions() -> Vec<String> {
    vec![
        "ListObjects".into(),
        "HeadObject".into(),
        "GetObject".into(),
        "PutObject".into(),
        "CopyObject".into(),
    ]
}

fn base_s3_actions_with_delete() -> Vec<String> {
    let mut base_s3actions = base_s3_actions();
    base_s3actions.extend(vec!["DeleteObject".into()]);
    base_s3actions
}

pub fn policy_os_7478_us_group_3_cjj0() -> Policy<ObjectStoragePolicy> {
    // s3://data-processing-data/bigdata/caojinjuan
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "c688c143-6575-4a54-b00b-e81f7dc22f3c".to_string(),
        name: "policy_os_7478_us_group_3_cjj0".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "ca368e22-4a2b-4daa-802b-112e3b81389d".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions_with_delete()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["data-processing-data".into()]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_3977_cn_group_3_shf0() -> Policy<ObjectStoragePolicy> {
    // s3://datalake-internal.patsnap.com-cn-northwest-1/tmp
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "1504a97e-bd1d-4e5a-9397-7f2f7764a188".to_string(),
        name: "policy_os_3977_cn_group_3_shf0".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "46cfbfb7-2104-4d09-96bd-1338a94366be".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions_with_delete()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["datalake-internal.patsnap.com-cn-northwest-1".into()]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_4258_cn_group_3_qwt0() -> Policy<ObjectStoragePolicy> {
    // cos://patsnap-country-source-1251949819/LEGAL/JP/REEXAM/DETAIL/
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "d3337c4b-33df-47be-980b-6dd31a67ca1b".to_string(),
        name: "policy_os_3977_cn_group_3_qwt0".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "8aaba6e7-0ad9-469f-9713-0c73c12b927e".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["patsnap-country-source-1251949819".into()]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_4258_us_group_3_fxd0() -> Policy<ObjectStoragePolicy> {
    // cos://patsnap-country-source-1251949819/LITIGATION
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "af6ac00e-914a-4683-9e84-04513f8c92d4".to_string(),
        name: "policy_os_4258_us_group_3_fxd0".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "19b81e6d-ea2f-44a6-ac36-74bdbee1190f".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["patsnap-country-source-1251949819".into()]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_0066_us_group_3_zsz0() -> Policy<ObjectStoragePolicy> {
    // s3://patsnap-country-source
    // s3://testpatsnapus
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "c9f3b3f1-3b1a-4b0f-8b1a-3b1a4b0f8b1a".to_string(),
        name: "policy_os_0066_us_group_3_zsz0".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "c9c250d6-95a9-4f22-be41-78bdae330d7a".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec![
                            "patsnap-country-source".into(),
                            "testpatsnapus".into(),
                        ]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_4258_us_group_3_zsz0() -> Policy<ObjectStoragePolicy> {
    // cos://patsnap-country-source-1251949819
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "b41a95b9-fbb7-41ca-9392-69fc6928f592".to_string(),
        name: "policy_os_4258_us_group_3_zsz0".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "053e952b-a03f-453d-b9a3-a6c9d784d740".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["patsnap-country-source-1251949819".into()]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_0066_us_group_3_whl0() -> Policy<ObjectStoragePolicy> {
    // s3://testpatsnapus
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "fca756ea-042d-40d9-857d-0861147cdae7".to_string(),
        name: "policy_os_0066_us_group_3_whl0".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "60b8ac47-2e10-4410-a593-8f4805e8074f".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["testpatsnapus".into()]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_7478_us_east00000_1_group_team_sa_dev() -> Policy<ObjectStoragePolicy> {
    // 7478 s3://pdf-patsnap-us-east-1/US/A1/20/20/03/29/65/5/US_20200329655_A1.pdf
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "357DD2B7-268D-49D2-858F-1CA574739CE0".to_string(),
        name: "policy_os_7478_us_east00000_1_group_sa_dev".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "A158CB2A-D56D-4F1B-9A87-635D2B789149".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["pdf-patsnap-us-east-1".into()]),
                        start_with: None,
                    }),
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        name: None,
                        tag: None,
                        effect: Some(Effect::allow()),
                    }]),
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_os_3977_cn_northwest_1_group_team_sa_dev() -> Policy<ObjectStoragePolicy> {
    // 3977 s3://data-pdf-cn-northwest-1/CN/A/11/50/67/44/8/CN_115067448_A.pdf
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "B64AC16F-D9AA-47AD-9DDD-30055FC0DD9C".to_string(),
        name: "policy_os_3977_cn_northwest_1_group_sa_dev".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "04DD8391-A013-405C-A3CD-2BC490DC6C27".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["data-pdf-cn-northwest-1".into()]),
                        start_with: None,
                    }),
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        name: None,
                        tag: None,
                        effect: Some(Effect::allow()),
                    }]),
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_os_7478_us_east00000_1_group_team_data_dev() -> Policy<ObjectStoragePolicy> {
    // s3://datalake-internal.patsnap.com-cn-northwest-1/tmp
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "D91C0510-BF5E-4050-9651-E27A0CD1ACD1".to_string(),
        name: "policy_os_7478_us_east00000_1_group_team_data_dev".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "363700F4-F590-42D8-B7FA-C44651DBFF91".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec![]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_3977_cn_northwest_1_group_team_data_dev() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "E8AD189E-3DDE-4C89-BD9D-3DC1676C2355".to_string(),
        name: "policy_os_3977_cn_northwest_1_group_team_data_dev".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "2CD54225-06EB-43C1-A276-05F05088E4FC".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["datalake-internal.patsnap.com-cn-northwest-1".into()]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_0066_us_east00000_1_group_team_data_dev() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "F06FA0B9-05E4-42DE-A4B1-74927303A54E".to_string(),
        name: "policy_os_0066_us_east00000_1_group_team_data_dev".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "ECCA678E-9892-4F3E-8879-7A8EC2E2F262".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec![
                            "patsnap-country-source".into(),
                            "testpatsnapus".into(),
                        ]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_4258_na_ashburn0000_group_team_data_dev() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "832F4043-C375-40FA-9FD4-1DE8B56E6821".to_string(),
        name: "policy_os_4258_na_ashburn0000_group_team_data_dev".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 1,
            id: "8E2B6016-78B1-494E-979E-701F4D4D33B8".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["patsnap-country-source-1251949819".into()]),
                        start_with: None,
                    }),
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }],
        ..Default::default()
    }
}

pub fn policy_os_7478_us_east00000_1_group_team_data_services() -> Policy<ObjectStoragePolicy> {
    // 7478 s3://discovery-attachment-us-east-1/data/drug_approvals_v2/pmda/pdf/*.pdf
    // 7478 s3://datalake-internal.patsnap.com/dpp/rd_process/test_parser_title_CN_v1_offline/
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "9d216c69-d9db-4720-8146-c5b2be6681bb".to_string(),
        name: "policy_os_7478_us_east00000_1_group_team_data_services".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "63ae2d4c-5165-47ef-bcd7-203fc83b130a".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec![
                            "discovery-attachment-us-east-1".into(),
                            "datalake-internal.patsnap.com".into(),
                        ]),
                        start_with: None,
                    }),
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        name: None,
                        tag: None,
                        effect: Some(Effect::allow()),
                    }]),
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_os_3977_cn_northwest_1_group_team_data_services() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "ba8cf2e6-ed3e-4c32-a054-48d86abee06e".to_string(),
        name: "policy_os_3977_cn_northwest_1_group_team_data_services".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "944bf4ba-7618-4d28-9eda-52f9ef1750f0".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec![
                            "data-pdf-cn-northwest-1".into(),
                            "data-page-cn-northwest-1".into(),
                            "data-page120-cn-northwest-1".into(),
                            "data-image-cn-northwest-1".into(),
                            "data-image120-cn-northwest-1".into(),
                            "data-fulltextimage-cn-northwest-1".into(),
                            "data-fulltextimage240-cn-northwest-1".into(),
                            "data-page-image-cn-northwest-1".into(),
                            "data-page-image240-cn-northwest-1".into(),
                        ]),
                        start_with: None,
                    }),
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        name: None,
                        tag: None,
                        effect: Some(Effect::allow()),
                    }]),
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_os_0066_us_east00000_1_group_team_data_services() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "00a85912-02cc-40cd-8f0c-baed0409f23b".to_string(),
        name: "policy_os_0066_us_east00000_1_group_team_data_services".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "1ff4803e-7745-47ad-a11e-f6c62595db49".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions_with_delete()),
                bucket: Bucket {
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_os_4258_na_ashburn0000_group_team_data_services() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "8be9190d-e7b1-4398-ae62-95afc5c69b0b".to_string(),
        name: "policy_os_4258_na_ashburn0000_group_team_data_services".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "81eec3f4-a4d4-46f0-97fe-10635a2f8632".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions_with_delete()),
                bucket: Bucket {
                    name: Some(Name {
                        eq: Some(vec!["patsnap-country-source-1251949819".into()]),
                        start_with: None,
                    }),
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_os_opst_for_all() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "05e75fbf-81dc-4962-9c78-9bab8d4fc926".to_string(),
        name: "policy_os_opst_for_all".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "23b22c98-b71b-41ad-93bf-fcdb72700d98".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions_with_delete()),
                bucket: Bucket {
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_os_7478_us_east00000_1_group_data_tmp() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "bd4558d4-c0d6-4045-afd5-902d6714c021".to_string(),
        name: "policy_os_7478_us_east00000_1_group_data_tmp".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "c114362f-aa1a-4cfc-8b5d-5eca3d37b068".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_os_3977_cn_northwest_1_group_data_tmp() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "35b1e920-0856-4965-aed4-7ced267ee4d5".to_string(),
        name: "policy_os_3977_cn_northwest_1_group_data_tmp".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "238925f9-9a61-4cc1-ac04-712c42f14ff6".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions()),
                bucket: Bucket {
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_os_group_allow_all() -> Policy<ObjectStoragePolicy> {
    Policy {
        kind: OBJECT_STORAGE.to_string(),
        version: 1,
        id: "5ADBA056-2EC1-4B6C-A8F4-B42245509180".to_string(),
        name: "policy_os_group_allow_all".to_string(),
        modeled_policy: vec![ObjectStoragePolicy {
            version: 0,
            id: "36EADFB4-E229-4D73-9D79-BA0BCE997DDB".to_string(),
            input_policy: ObjectStorageInputPolicy {
                actions: Some(base_s3_actions_with_delete()),
                bucket: Bucket {
                    tag: None,
                    effect: Some(Effect::allow()),
                    keys: Some(vec![Key {
                        effect: Some(Effect::allow()),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            },
            output_policy: None,
        }],
        ..Default::default()
    }
}

pub fn policy_allow_private_ip() -> Policy<ConditionPolicy> {
    Policy {
        kind: CONDITION.to_string(),
        version: 0,
        id: "EA70AC12-ACB9-438A-BC3A-904FBCE4DD22".to_string(),
        name: "policy_allow_private_ip".to_string(),
        modeled_policy: vec![ConditionPolicy {
            version: 0,
            id: "A7013E3B-A225-4AF8-8747-A0745B794104".to_string(),
            range: ConditionRange {
                group_ids: None,
                from: Some(Range {
                    ip_cidr: Some(private_ip_cidr()),
                    region: None,
                    env: None,
                }),
                proxy: None,
                to: None,
            },
            effect: Effect::allow(),
        }],
    }
}

pub fn policy_proxy_prefilter_local_qa() -> Policy<ConditionPolicy> {
    Policy {
        kind: CONDITION.to_string(),
        version: 0,
        id: "7CD016F9-C1AE-456D-BA65-4C038E977082".to_string(),
        name: "policy_proxy_prefilter_local_qa".to_string(),
        modeled_policy: vec![ConditionPolicy {
            version: 0,
            id: "0EC23518-E6EE-467D-9518-397580DD84F9".to_string(),
            range: ConditionRange {
                group_ids: Some(vec![
                    group_team_sa_dev().id,
                    group_team_data_dev().id,
                    group_data_tmp().id,
                ]),
                proxy: Some(Range {
                    ip_cidr: None,
                    region: Some(vec![Local.into()]),
                    env: None,
                }),
                ..Default::default()
            },
            effect: Effect::allow(),
        }],
    }
}

pub fn policy_proxy_prefilter_non_local() -> Policy<ConditionPolicy> {
    Policy {
        kind: CONDITION.to_string(),
        version: 0,
        id: "6B7D5432-5E07-4467-A9C9-61EA71466AD0".to_string(),
        name: "policy_proxy_prefilter_non_local".to_string(),
        modeled_policy: vec![ConditionPolicy {
            version: 0,
            id: "683BC56A-7357-4E8F-A99D-22C6BEEC19D0".to_string(),
            range: ConditionRange {
                group_ids: Some(vec![group_team_data_services().id]),
                proxy: Some(Range {
                    ip_cidr: None,
                    region: Some(vec![
                        UsEast1.into(),
                        CnNorthwest1.into(),
                        NaAshburn.into(),
                        ApShanghai.into(),
                    ]),
                    env: None,
                }),
                ..Default::default()
            },
            effect: Effect::allow(),
        }],
    }
}

pub fn policy_proxy_prefilter_us_east_1_prod() -> Policy<ConditionPolicy> {
    Policy {
        kind: CONDITION.to_string(),
        version: 0,
        id: "600DBBF0-3A26-4361-8150-F963AAD1CF40".to_string(),
        name: "policy_proxy_prefilter_us_east_1_prod".to_string(),
        modeled_policy: vec![ConditionPolicy {
            version: 0,
            id: "4CC48917-FC70-4383-86CF-A11350CF2CD7".to_string(),
            range: ConditionRange {
                group_ids: Some(vec![]),
                proxy: Some(Range {
                    ip_cidr: None,
                    region: Some(vec![UsEast1.to_string()]),
                    env: Some(vec![Prod.to_string()]),
                }),
                ..Default::default()
            },
            effect: Effect::allow(),
        }],
    }
}

pub fn make_policies_object_storage() -> Vec<Policy<ObjectStoragePolicy>> {
    vec![
        policy_os_7478_us_group_3_cjj0(),
        policy_os_3977_cn_group_3_shf0(),
        policy_os_4258_cn_group_3_qwt0(),
        policy_os_4258_us_group_3_fxd0(),
        policy_os_0066_us_group_3_zsz0(),
        policy_os_4258_us_group_3_zsz0(),
        policy_os_0066_us_group_3_whl0(),
        policy_os_7478_us_east00000_1_group_team_sa_dev(),
        policy_os_3977_cn_northwest_1_group_team_sa_dev(),
        policy_os_7478_us_east00000_1_group_team_data_dev(),
        policy_os_3977_cn_northwest_1_group_team_data_dev(),
        policy_os_0066_us_east00000_1_group_team_data_dev(),
        policy_os_4258_na_ashburn0000_group_team_data_dev(),
        policy_os_7478_us_east00000_1_group_team_data_services(),
        policy_os_3977_cn_northwest_1_group_team_data_services(),
        policy_os_0066_us_east00000_1_group_team_data_services(),
        policy_os_4258_na_ashburn0000_group_team_data_services(),
        policy_os_opst_for_all(),
        policy_os_7478_us_east00000_1_group_data_tmp(),
        policy_os_3977_cn_northwest_1_group_data_tmp(),
        policy_os_group_allow_all(),
    ]
}

pub fn make_policies_condition() -> Vec<Policy<ConditionPolicy>> {
    vec![
        policy_allow_private_ip(),
        policy_proxy_prefilter_local_qa(),
        policy_proxy_prefilter_non_local(),
        policy_proxy_prefilter_us_east_1_prod(),
    ]
}

pub fn make_user_group_relationships() -> Vec<UserGroupRelationship> {
    let group_team_data_svcs: Vec<UserGroupRelationship> = vec![
        user_svcs_d_data_rd_processing_batch_qa(),
        user_svcs_d_data_dwc_script(),
        user_svcs_d_data_image_sync_recover(),
    ]
    .into_iter()
    .map(|u| UserGroupRelationship {
        id: Uuid::new_v4().to_string(),
        user_id: u.id,
        group_id: group_team_data_services().id,
    })
    .collect();
    let group_team_sa_dev: Vec<UserGroupRelationship> = vec![user_3_cjj0(), user_3_wwt0()]
        .into_iter()
        .map(|u| UserGroupRelationship {
            id: Uuid::new_v4().to_string(),
            user_id: u.id,
            group_id: group_team_sa_dev().id,
        })
        .collect();
    let group_team_data_dev: Vec<UserGroupRelationship> = vec![
        // data
        user_3_cyy0(),
        user_3_shf0(),
        // data qa
        user_3_qwt0(),
        user_3_zsz0(),
        user_3_whl0(),
        user_3_fxd0(),
    ]
    .into_iter()
    .map(|u| UserGroupRelationship {
        id: Uuid::new_v4().to_string(),
        user_id: u.id,
        group_id: group_team_data_dev().id,
    })
    .collect();
    let group_persons = vec![
        UserGroupRelationship {
            id: "e0e80393-f471-4fd4-986b-1cf26d583534".to_string(),
            user_id: user_3_cjj0().id,
            group_id: group_3_cjj0().id,
        },
        UserGroupRelationship {
            id: "7f0a03c0-5549-4416-9a05-bb1dd04eb90b".to_string(),
            user_id: user_3_shf0().id,
            group_id: group_3_shf0().id,
        },
        UserGroupRelationship {
            id: "6850beed-4170-404b-9d21-a724de852e5b".to_string(),
            user_id: user_3_qwt0().id,
            group_id: group_3_qwt0().id,
        },
        UserGroupRelationship {
            id: "85091850-22fe-47dd-82a3-a3f87e85bc33".to_string(),
            user_id: user_3_fxd0().id,
            group_id: group_3_fxd0().id,
        },
        UserGroupRelationship {
            id: "50905fd4-4b1a-4470-88a4-f7acf3f5ee91".to_string(),
            user_id: user_3_zsz0().id,
            group_id: group_3_zsz0().id,
        },
        UserGroupRelationship {
            id: "7cd34871-7454-4bb6-b246-7f62b95cff18".to_string(),
            user_id: user_3_whl0().id,
            group_id: group_3_whl0().id,
        },
    ];
    let group_opst = vec![UserGroupRelationship {
        id: "98553c33-2f33-4820-a610-c23acce7bb6a".to_string(),
        user_id: user_svcs_opst().id,
        group_id: group_svcs_opst().id,
    }];
    let group_data_tmp = vec![
        UserGroupRelationship {
            id: "b2e23c95-a130-4912-a28d-dcae4e19cb61".to_string(),
            user_id: user_team_data_tmp().id,
            group_id: group_data_tmp().id,
        },
        UserGroupRelationship {
            id: "f6d0c87c-9962-45dd-ad9f-64818f75e6b4".to_string(),
            user_id: user_svcs_data_tmp().id,
            group_id: group_data_tmp().id,
        },
    ];
    let group_allow_all = vec![UserGroupRelationship {
        id: "0F3FD5AD-90D1-4956-9323-3087EE226655".to_string(),
        user_id: user_dev().id,
        group_id: group_allow_all().id,
    }];
    let mut relations = vec![];
    relations.extend(group_team_data_dev);
    relations.extend(group_team_sa_dev);
    relations.extend(group_team_data_svcs);
    relations.extend(group_persons);
    relations.extend(group_opst);
    // TODO: remove data tmp contents completely
    // ugrs.extend(group_data_tmp);
    relations.extend(group_allow_all);
    relations
}

pub fn make_policy_relationships() -> Vec<PolicyRelationship> {
    vec![
        PolicyRelationship {
            id: "cff6f6d1-4d4a-4bdc-9636-4712da88962f".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_3_cjj0().id),
            role_id: None,
            account_id: AWS_PROD_7478.to_string(),
            region: Region::UsEast1.into(),
            policy_id: policy_os_7478_us_group_3_cjj0().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "c0b0b0b0-4d4a-4bdc-9636-4712da88962f".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_3_shf0().id),
            role_id: None,
            account_id: AWS_PROD_3977.to_string(),
            region: Region::CnNorthwest1.into(),
            policy_id: policy_os_3977_cn_group_3_shf0().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80826".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_3_qwt0().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: Region::NaAshburn.into(),
            policy_id: policy_os_4258_cn_group_3_qwt0().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80827".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_3_fxd0().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: Region::NaAshburn.into(),
            policy_id: policy_os_4258_us_group_3_fxd0().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80828".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_3_zsz0().id),
            role_id: None,
            account_id: AWS_DATA_0066.to_string(),
            region: Region::UsEast1.into(),
            policy_id: policy_os_0066_us_group_3_zsz0().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80829".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_3_zsz0().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: Region::NaAshburn.into(),
            policy_id: policy_os_4258_us_group_3_zsz0().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80830".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_3_whl0().id),
            role_id: None,
            account_id: AWS_DATA_0066.to_string(),
            region: Region::UsEast1.into(),
            policy_id: policy_os_0066_us_group_3_whl0().id,
            ..Default::default()
        },
        // team devs
        PolicyRelationship {
            id: "D62CFB11-A8B9-4036-A5B0-52AF279F08E3".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_sa_dev().id),
            role_id: None,
            account_id: AWS_PROD_7478.to_string(),
            region: Region::UsEast1.into(),
            policy_id: policy_os_7478_us_east00000_1_group_team_sa_dev().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "35318B88-38A0-4FAB-8C5A-7CF9EBA0A6E9".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_sa_dev().id),
            role_id: None,
            account_id: AWS_PROD_3977.to_string(),
            region: Region::CnNorthwest1.into(),
            policy_id: policy_os_3977_cn_northwest_1_group_team_sa_dev().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "61636630-4395-4ACC-8A88-AE47A322AA3E".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_data_dev().id),
            role_id: None,
            account_id: AWS_PROD_7478.to_string(),
            region: Region::UsEast1.into(),
            policy_id: policy_os_7478_us_east00000_1_group_team_data_dev().id,
        },
        PolicyRelationship {
            id: "D62CFB11-A8B9-4036-A5B0-52AF279F08E4".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_data_dev().id),
            role_id: None,
            account_id: AWS_PROD_3977.to_string(),
            region: Region::CnNorthwest1.into(),
            policy_id: policy_os_3977_cn_northwest_1_group_team_data_dev().id,
        },
        PolicyRelationship {
            id: "38F8AD89-C117-4933-8CBE-DCD012E41A8B".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_data_dev().id),
            role_id: None,
            account_id: AWS_DATA_0066.to_string(),
            region: Region::UsEast1.into(),
            policy_id: policy_os_0066_us_east00000_1_group_team_data_dev().id,
        },
        PolicyRelationship {
            id: "7F838823-BBA1-44BD-AFD0-892738C64B43".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_data_dev().id),
            role_id: None,
            account_id: TENCENT_4258.to_string(),
            region: Region::NaAshburn.into(),
            policy_id: policy_os_4258_na_ashburn0000_group_team_data_dev().id,
        },
        // svcs
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80831".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: AWS_PROD_7478.to_string(),
            region: Region::UsEast1.into(),
            policy_id: policy_os_7478_us_east00000_1_group_team_data_services().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80832".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: AWS_PROD_3977.to_string(),
            region: Region::CnNorthwest1.into(),
            policy_id: policy_os_3977_cn_northwest_1_group_team_data_services().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80833".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: AWS_DATA_0066.to_string(),
            region: Region::UsEast1.into(),
            policy_id: policy_os_0066_us_east00000_1_group_team_data_services().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80834".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: Region::NaAshburn.into(),
            policy_id: policy_os_4258_na_ashburn0000_group_team_data_services().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80835".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: TENCENT_4258.to_string(),
            region: Region::ApShanghai.into(),
            policy_id: policy_os_4258_na_ashburn0000_group_team_data_services().id,
            ..Default::default()
        },
        // opst
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80836".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_svcs_opst().id),
            role_id: None,
            account_id: ANY.to_string(),
            region: Region::Any.into(),
            policy_id: policy_os_opst_for_all().id,
            ..Default::default()
        },
        // data tmp
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80842".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_data_tmp().id),
            role_id: None,
            account_id: AWS_PROD_7478.to_string(),
            region: Region::UsEast1.into(),
            policy_id: policy_os_7478_us_east00000_1_group_data_tmp().id,
            ..Default::default()
        },
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80843".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_data_tmp().id),
            role_id: None,
            account_id: AWS_PROD_3977.to_string(),
            region: Region::CnNorthwest1.into(),
            policy_id: policy_os_3977_cn_northwest_1_group_data_tmp().id,
            ..Default::default()
        },
        // allow all
        PolicyRelationship {
            id: "ae14e161-3357-46a5-b771-31abfdf80844".to_string(),
            policy_model: OBJECT_STORAGE.to_string(),
            user_id: None,
            group_id: Some(group_allow_all().id),
            role_id: None,
            account_id: ANY.to_string(),
            region: Region::Any.into(),
            policy_id: policy_os_group_allow_all().id,
            ..Default::default()
        },
        // private ip
        PolicyRelationship {
            id: "5DB4761D-7371-4D0F-AE41-7B16B356101C".to_string(),
            policy_model: CONDITION.to_string(),
            user_id: Some(ANY.into()),
            group_id: Some(ANY.into()),
            role_id: Some(ANY.into()),
            account_id: ANY.to_string(),
            region: Region::Any.into(),
            policy_id: policy_allow_private_ip().id,
            ..Default::default()
        },
        // proxy condition
        PolicyRelationship {
            id: "5DB4761D-7371-4D0F-AE41-7B16B356101D".to_string(),
            policy_model: CONDITION.to_string(),
            user_id: Some(ANY.into()),
            group_id: Some(ANY.into()),
            role_id: Some(ANY.into()),
            account_id: ANY.to_string(),
            region: Region::Any.into(),
            policy_id: policy_proxy_prefilter_local_qa().id,
        },
        PolicyRelationship {
            id: "CF00A6A2-3744-48C8-B30B-D015F9E269CC".to_string(),
            policy_model: CONDITION.to_string(),
            user_id: Some(ANY.into()),
            group_id: Some(ANY.into()),
            role_id: Some(ANY.into()),
            account_id: ANY.to_string(),
            region: Region::Any.into(),
            policy_id: policy_proxy_prefilter_non_local().id,
        },
    ]
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct S3Config {
    pub proxy_hosts: S3HostDomains,
}

pub fn make_s3_config() -> S3Config {
    S3Config {
        proxy_hosts: S3HostDomains {
            domains: vec![
                "internal.s3-proxy.patsnap.info".into(),
                "us-east-1.s3-proxy.patsnap.info".into(),
                "cn-northwest-1.s3-proxy.patsnap.info".into(),
                "na-ashburn.s3-proxy.patsnap.info".into(),
                "ap-shanghai.s3-proxy.patsnap.info".into(),
                "local.s3-proxy.patsnap.info".into(),
                "s3-proxy.dev".into(),
            ],
        },
    }
}

fn ser<T: ?Sized + ser::Serialize>(value: &T) -> String {
    serde_yaml::to_string(value).unwrap()
}

#[test]
fn write_prod() {
    write_all(&redis::Client::open("redis://dev-redis.patsnap.info:30070/1").unwrap())
}

#[test]
fn write_dev() {
    write_all(&redis::Client::open("redis://localhost/1").unwrap())
}

fn write_all(client: &Client) {
    let _accounts = ser(&make_accounts());
    let users = ser(&make_users());
    let groups = ser(&make_groups());
    let policies_object_storage = ser(&make_policies_object_storage());
    let policies_condition = ser(&make_policies_condition());

    let user_group_relationships = ser(&make_user_group_relationships());
    let policy_relationships = ser(&make_policy_relationships());
    let s3_config = ser(&make_s3_config());

    write(client, "accounts", read_last_version(client, "accounts"));
    write(client, "users", users);
    write(client, "groups", groups);
    write(client, "policies:ObjectStorage", policies_object_storage);
    write(client, "policies:Condition", policies_condition);

    write(client, "user_group_relationships", user_group_relationships);
    write(client, "policy_relationships", policy_relationships);
    write(client, "extended_config:s3", s3_config);
}

pub fn write(client: &Client, key: &str, value: String) {
    let mut con = client.get_connection().unwrap();
    let key = format!("piam:{VERSION}:{key}");
    con.set(key, value).unwrap()
}

pub fn read_last_version(client: &Client, key: &str) -> String {
    let mut con = client.get_connection().unwrap();
    let last_version = VERSION.replace('v', "");
    let last_version = last_version.parse::<i32>().unwrap() - 1;
    let key = format!("piam:v{last_version}:{key}");
    con.get(key).unwrap()
}
