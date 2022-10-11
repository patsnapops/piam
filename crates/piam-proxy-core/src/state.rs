use std::{fmt::Debug, sync::Arc};

use serde::de::DeserializeOwned;
use tokio::sync::RwLock;

use crate::{
    policy::PolicyContainer,
    principal::PrincipalContainer,
    rule_api::{get_policies, get_principals},
};

pub type SharedState<S> = Arc<RwLock<ProxyState<S>>>;

#[derive(Debug, Default)]
pub struct ProxyState<S: Debug> {
    pub principal_container: PrincipalContainer,
    pub policy_container: PolicyContainer<S>,
}

pub struct StateUpdater<S: Debug> {
    kind: String,
    pub state: SharedState<S>,
}

impl<S: DeserializeOwned + Debug + Default + Sync + 'static> StateUpdater<S> {
    pub async fn new_with_kind(kind: &str) -> Self {
        StateUpdater {
            kind: kind.to_string(),
            state: Arc::new(RwLock::new(ProxyState::default())),
        }
    }

    pub async fn update(&self) {
        let mut guard = self.state.write().await;
        guard.principal_container = get_principals().await;
        guard.policy_container = get_policies(&self.kind).await;
    }
}
