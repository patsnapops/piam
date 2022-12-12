use piam_proxy_core::{
    account::aws::AwsAccount,
    config::{AP_SHANGHAI, CN_NORTHWEST_1, NA_ASHBURN, US_EAST_1},
    effect::Effect,
    group::Group,
    policy::{
        object_storage_policy::{Bucket, Key, ObjectStorageInputStatement, ObjectStorageStatement},
        Name, Policy,
    },
    principal::{User, UserKind},
    relation_model::{PolicyRelationship, UserGroupRelationship},
};
use redis::Commands;
use serde::{ser, Deserialize, Serialize};

pub fn make_accounts() -> Vec<AwsAccount> {
    let account_cn_aws_dev_9554 = AwsAccount {
        id: "cn_aws_dev_9554".into(),
        code: "9554".into(),
        access_key: "AKIA545RXJQZN5UDTZZZ".into(),
        secret_key: "".into(),
        comment: "".to_string(),
    };
    let account_cn_aws_prod_3977 = AwsAccount {
        id: "cn_aws_prod_3977".into(),
        code: "3977".into(),
        access_key: "AKIAVZG6PPVKGB77FSFI".into(),
        secret_key: "".into(),
        comment: "".to_string(),
    };
    let account_us_aws_prod_7478 = AwsAccount {
        id: "us_aws_prod_7478".into(),
        code: "7478".into(),
        access_key: "AKIA24IGUMII4I3EGYOU".into(),
        secret_key: "".into(),
        comment: "".to_string(),
    };
    let account_us_aws_data_0066 = AwsAccount {
        id: "us_aws_data_0066".into(),
        code: "0066".into(),
        access_key: "AKIAQDDYEQIRTBKNVKFR".into(),
        secret_key: "".into(),
        comment: "".to_string(),
    };
    let account_cn_tencent_4258 = AwsAccount {
        id: "cn_tencent_4258".to_string(),
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

pub fn make_groups() -> Vec<Group> {
    vec![
        group_3_cjj0(),
        group_3_shf0(),
        group_3_qwt0(),
        group_3_fxd0(),
        group_3_zsz0(),
        group_team_data_services(),
        group_svcs_opst(),
        group_data_tmp(),
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

pub fn policy_os_7478_us_group_3_cjj0() -> Policy<ObjectStorageStatement> {
    // s3://data-processing-data/bigdata/caojinjuan
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "c688c143-6575-4a54-b00b-e81f7dc22f3c".to_string(),
        name: "policy_os_7478_us_group_3_cjj0".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 1,
            id: "ca368e22-4a2b-4daa-802b-112e3b81389d".to_string(),
            input_statement: ObjectStorageInputStatement {
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
    }
}

pub fn policy_os_3977_cn_group_3_shf0() -> Policy<ObjectStorageStatement> {
    // s3://datalake-internal.patsnap.com-cn-northwest-1/tmp
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "1504a97e-bd1d-4e5a-9397-7f2f7764a188".to_string(),
        name: "policy_os_3977_cn_group_3_shf0".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 1,
            id: "46cfbfb7-2104-4d09-96bd-1338a94366be".to_string(),
            input_statement: ObjectStorageInputStatement {
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
    }
}

pub fn policy_os_3977_cn_group_3_qwt0() -> Policy<ObjectStorageStatement> {
    // cos://patsnap-country-source-1251949819/LEGAL/JP/REEXAM/DETAIL/
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "d3337c4b-33df-47be-980b-6dd31a67ca1b".to_string(),
        name: "policy_os_3977_cn_group_3_qwt0".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 1,
            id: "8aaba6e7-0ad9-469f-9713-0c73c12b927e".to_string(),
            input_statement: ObjectStorageInputStatement {
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
    }
}

pub fn policy_os_4258_us_group_3_fxd0() -> Policy<ObjectStorageStatement> {
    // cos://patsnap-country-source-1251949819/LITIGATION
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "af6ac00e-914a-4683-9e84-04513f8c92d4".to_string(),
        name: "policy_os_4258_us_group_3_fxd0".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 1,
            id: "19b81e6d-ea2f-44a6-ac36-74bdbee1190f".to_string(),
            input_statement: ObjectStorageInputStatement {
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
    }
}

pub fn policy_os_0066_us_group_3_zsz0() -> Policy<ObjectStorageStatement> {
    // s3://patsnap-country-source
    // s3://testpatsnapus
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "c9f3b3f1-3b1a-4b0f-8b1a-3b1a4b0f8b1a".to_string(),
        name: "policy_os_0066_us_group_3_zsz0".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 1,
            id: "c9c250d6-95a9-4f22-be41-78bdae330d7a".to_string(),
            input_statement: ObjectStorageInputStatement {
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
    }
}

pub fn policy_os_4258_us_group_3_zsz0() -> Policy<ObjectStorageStatement> {
    // cos://patsnap-country-source-1251949819
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "b41a95b9-fbb7-41ca-9392-69fc6928f592".to_string(),
        name: "policy_os_4258_us_group_3_zsz0".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 1,
            id: "053e952b-a03f-453d-b9a3-a6c9d784d740".to_string(),
            input_statement: ObjectStorageInputStatement {
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
    }
}

pub fn policy_os_7478_us_east00000_1_group_team_data_services() -> Policy<ObjectStorageStatement> {
    // 7478 s3://discovery-attachment-us-east-1/data/drug_approvals_v2/pmda/pdf/*.pdf
    // 7478 s3://datalake-internal.patsnap.com/dpp/rd_process/test_parser_title_CN_v1_offline/
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "9d216c69-d9db-4720-8146-c5b2be6681bb".to_string(),
        name: "policy_os_7478_us_east00000_1_group_team_data_services".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 0,
            id: "63ae2d4c-5165-47ef-bcd7-203fc83b130a".to_string(),
            input_statement: ObjectStorageInputStatement {
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
            output_statement: None,
        }],
    }
}

pub fn policy_os_3977_cn_northwest_1_group_team_data_services() -> Policy<ObjectStorageStatement> {
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "ba8cf2e6-ed3e-4c32-a054-48d86abee06e".to_string(),
        name: "policy_os_3977_cn_northwest_1_group_team_data_services".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 0,
            id: "944bf4ba-7618-4d28-9eda-52f9ef1750f0".to_string(),
            input_statement: ObjectStorageInputStatement {
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
            output_statement: None,
        }],
    }
}

pub fn policy_os_0066_us_east00000_1_group_team_data_services() -> Policy<ObjectStorageStatement> {
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "00a85912-02cc-40cd-8f0c-baed0409f23b".to_string(),
        name: "policy_os_0066_us_east00000_1_group_team_data_services".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 0,
            id: "1ff4803e-7745-47ad-a11e-f6c62595db49".to_string(),
            input_statement: ObjectStorageInputStatement {
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
            output_statement: None,
        }],
    }
}

pub fn policy_os_4258_na_ashburn0000_group_team_data_services() -> Policy<ObjectStorageStatement> {
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "8be9190d-e7b1-4398-ae62-95afc5c69b0b".to_string(),
        name: "policy_os_4258_na_ashburn0000_group_team_data_services".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 0,
            id: "81eec3f4-a4d4-46f0-97fe-10635a2f8632".to_string(),
            input_statement: ObjectStorageInputStatement {
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
            output_statement: None,
        }],
    }
}

pub fn policy_os_opst_for_all() -> Policy<ObjectStorageStatement> {
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "05e75fbf-81dc-4962-9c78-9bab8d4fc926".to_string(),
        name: "policy_os_opst_for_all".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 0,
            id: "23b22c98-b71b-41ad-93bf-fcdb72700d98".to_string(),
            input_statement: ObjectStorageInputStatement {
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
            output_statement: None,
        }],
    }
}

pub fn policy_os_7478_us_east00000_1_group_data_tmp() -> Policy<ObjectStorageStatement> {
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "bd4558d4-c0d6-4045-afd5-902d6714c021".to_string(),
        name: "policy_os_7478_us_east00000_1_group_data_tmp".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 0,
            id: "c114362f-aa1a-4cfc-8b5d-5eca3d37b068".to_string(),
            input_statement: ObjectStorageInputStatement {
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
            output_statement: None,
        }],
    }
}

pub fn policy_os_3977_cn_northwest_1_group_data_tmp() -> Policy<ObjectStorageStatement> {
    Policy {
        kind: "ObjectStorage".to_string(),
        version: 1,
        id: "35b1e920-0856-4965-aed4-7ced267ee4d5".to_string(),
        name: "policy_os_3977_cn_northwest_1_group_data_tmp".to_string(),
        conditions: None,
        statements: vec![ObjectStorageStatement {
            version: 0,
            id: "238925f9-9a61-4cc1-ac04-712c42f14ff6".to_string(),
            input_statement: ObjectStorageInputStatement {
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
            output_statement: None,
        }],
    }
}

pub fn make_policies_object_storage() -> Vec<Policy<ObjectStorageStatement>> {
    vec![
        policy_os_7478_us_group_3_cjj0(),
        policy_os_3977_cn_group_3_shf0(),
        policy_os_3977_cn_group_3_qwt0(),
        policy_os_4258_us_group_3_fxd0(),
        policy_os_0066_us_group_3_zsz0(),
        policy_os_4258_us_group_3_zsz0(),
        policy_os_7478_us_east00000_1_group_team_data_services(),
        policy_os_3977_cn_northwest_1_group_team_data_services(),
        policy_os_0066_us_east00000_1_group_team_data_services(),
        policy_os_4258_na_ashburn0000_group_team_data_services(),
        policy_os_opst_for_all(),
        policy_os_7478_us_east00000_1_group_data_tmp(),
        policy_os_3977_cn_northwest_1_group_data_tmp(),
    ]
}

pub fn make_user_group_relationships() -> Vec<UserGroupRelationship> {
    let group_team_data_services: Vec<UserGroupRelationship> = vec![
        user_svcs_d_data_rd_processing_batch_qa(),
        user_svcs_d_data_dwc_script(),
        user_svcs_d_data_image_sync_recover(),
    ]
    .into_iter()
    .map(|u| UserGroupRelationship {
        user_id: u.id,
        group_id: group_team_data_services().id,
    })
    .collect();
    let group_persons = vec![
        UserGroupRelationship {
            user_id: user_3_cjj0().id,
            group_id: group_3_cjj0().id,
        },
        UserGroupRelationship {
            user_id: user_3_shf0().id,
            group_id: group_3_shf0().id,
        },
        UserGroupRelationship {
            user_id: user_3_qwt0().id,
            group_id: group_3_qwt0().id,
        },
        UserGroupRelationship {
            user_id: user_3_fxd0().id,
            group_id: group_3_fxd0().id,
        },
        UserGroupRelationship {
            user_id: user_3_zsz0().id,
            group_id: group_3_zsz0().id,
        },
    ];
    let group_opst = vec![UserGroupRelationship {
        user_id: user_svcs_opst().id,
        group_id: group_svcs_opst().id,
    }];
    let group_data_tmp = vec![
        UserGroupRelationship {
            user_id: user_team_data_tmp().id,
            group_id: group_data_tmp().id,
        },
        UserGroupRelationship {
            user_id: user_svcs_data_tmp().id,
            group_id: group_data_tmp().id,
        },
    ];
    let mut ugrs = vec![];
    ugrs.extend(group_team_data_services);
    ugrs.extend(group_persons);
    ugrs.extend(group_opst);
    ugrs.extend(group_data_tmp);
    ugrs
}

pub fn make_policy_relationships() -> Vec<PolicyRelationship> {
    vec![
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_3_cjj0().id),
            role_id: None,
            account_id: "us_aws_prod_7478".to_string(),
            region: US_EAST_1.to_string(),
            policy_id: policy_os_7478_us_group_3_cjj0().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_3_shf0().id),
            role_id: None,
            account_id: "cn_aws_prod_3977".to_string(),
            region: CN_NORTHWEST_1.to_string(),
            policy_id: policy_os_3977_cn_group_3_shf0().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_3_qwt0().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: NA_ASHBURN.to_string(),
            policy_id: policy_os_3977_cn_group_3_qwt0().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_3_fxd0().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: NA_ASHBURN.to_string(),
            policy_id: policy_os_4258_us_group_3_fxd0().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_3_zsz0().id),
            role_id: None,
            account_id: "us_aws_prod_0066".to_string(),
            region: US_EAST_1.to_string(),
            policy_id: policy_os_0066_us_group_3_zsz0().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_3_zsz0().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: NA_ASHBURN.to_string(),
            policy_id: policy_os_4258_us_group_3_zsz0().id,
        },
        // svcs
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: "us_aws_prod_7478".to_string(),
            region: US_EAST_1.to_string(),
            policy_id: policy_os_7478_us_east00000_1_group_team_data_services().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: "cn_aws_prod_3977".to_string(),
            region: CN_NORTHWEST_1.to_string(),
            policy_id: policy_os_3977_cn_northwest_1_group_team_data_services().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: "us_aws_prod_0066".to_string(),
            region: US_EAST_1.to_string(),
            policy_id: policy_os_0066_us_east00000_1_group_team_data_services().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: NA_ASHBURN.to_string(),
            policy_id: policy_os_4258_na_ashburn0000_group_team_data_services().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_team_data_services().id),
            role_id: None,
            account_id: "cn_tencent_4258".to_string(),
            region: AP_SHANGHAI.to_string(),
            policy_id: policy_os_4258_na_ashburn0000_group_team_data_services().id,
        },
        // opst
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_svcs_opst().id),
            role_id: None,
            account_id: "us_aws_prod_7478".to_string(),
            region: US_EAST_1.to_string(),
            policy_id: policy_os_opst_for_all().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_svcs_opst().id),
            role_id: None,
            account_id: "us_aws_prod_0066".to_string(),
            region: US_EAST_1.to_string(),
            policy_id: policy_os_opst_for_all().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_svcs_opst().id),
            role_id: None,
            account_id: "cn_aws_prod_3977".to_string(),
            region: CN_NORTHWEST_1.to_string(),
            policy_id: policy_os_opst_for_all().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_svcs_opst().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: AP_SHANGHAI.to_string(),
            policy_id: policy_os_opst_for_all().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_svcs_opst().id),
            role_id: None,
            account_id: "us_tencent_4258".to_string(),
            region: "na-na-ashburn".to_string(),
            policy_id: policy_os_opst_for_all().id,
        },
        // data tmp
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_data_tmp().id),
            role_id: None,
            account_id: "us_aws_prod_7478".to_string(),
            region: US_EAST_1.to_string(),
            policy_id: policy_os_7478_us_east00000_1_group_data_tmp().id,
        },
        PolicyRelationship {
            policy_model: "ObjectStorage".to_string(),
            user_id: None,
            group_id: Some(group_data_tmp().id),
            role_id: None,
            account_id: "cn_aws_prod_3977".to_string(),
            region: CN_NORTHWEST_1.to_string(),
            policy_id: policy_os_3977_cn_northwest_1_group_data_tmp().id,
        },
    ]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct S3Config {
    pub proxy_hosts: Vec<String>,
}

pub fn make_s3_config() -> S3Config {
    S3Config {
        proxy_hosts: vec![
            "internal.s3-proxy.patsnap.info".into(),
            "us-east-1.s3-proxy.patsnap.info".into(),
            "cn-northwest-1.s3-proxy.patsnap.info".into(),
            "na-ashburn.s3-proxy.patsnap.info".into(),
            "ap-shanghai.s3-proxy.patsnap.info".into(),
            "local.s3-proxy.patsnap.info".into(),
            "piam-s3-proxy.dev".into(),
        ],
    }
}

fn ser<T: ?Sized + ser::Serialize>(value: &T) -> String {
    serde_yaml::to_string(value).unwrap()
}

#[test]
fn write_all() {
    let _accounts = ser(&make_accounts());
    let users = ser(&make_users());
    let groups = ser(&make_groups());
    let policies_object_storage = ser(&make_policies_object_storage());

    let user_group_relationships = ser(&make_user_group_relationships());
    let policy_relationships = ser(&make_policy_relationships());
    let s3_config = ser(&make_s3_config());

    // write("accounts", _accounts);
    write("users", users);
    write("groups", groups);
    write("policies:ObjectStorage", policies_object_storage);

    write("user_group_relationships", user_group_relationships);
    write("policy_relationships", policy_relationships);
    write("extended_config:s3", s3_config);
}

pub fn write(key: &str, value: String) {
    let client = redis::Client::open("redis://dev-redis.patsnap.info:30070/1").unwrap();
    let client = redis::Client::open("redis://localhost/1").unwrap();
    let mut con = client.get_connection().unwrap();
    let key = format!("piam:{}", key);
    con.set(key, value).unwrap()
}
