pub mod logger;

pub mod manager_api {
    pub const VERSION: &str = "v2";

    pub const ACCOUNTS: &str = "accounts";

    pub const USERS: &str = "users";

    pub const GROUPS: &str = "groups";

    pub const POLICIES: &str = "policies";
    /// for manager use only
    pub const POLICY_MODEL: &str = "policy_model";
    pub const CONDITION_MODEL: &str = "Condition";

    pub const USER_GROUP_RELATIONSHIPS: &str = "user_group_relationships";

    pub const POLICY_RELATIONSHIPS: &str = "policy_relationships";

    pub const EXTENDED_CONFIG: &str = "extended_config";
    /// for manager use only
    pub const CONFIG_TYPE: &str = "config_type";

    /// for proxy use only
    pub fn policies_path(policy_model: &str) -> String {
        format!("{}/{}", POLICIES, policy_model)
    }

    /// for proxy use only
    pub fn extended_config_path(config_type: &str) -> String {
        format!("{}/{}", EXTENDED_CONFIG, config_type)
    }
}

pub const ANY: &str = "any";
