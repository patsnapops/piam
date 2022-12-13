use std::{fmt::Debug, sync::Arc, time::Instant};

use arc_swap::ArcSwap;
use async_trait::async_trait;
use log::error;
use piam_tracing::logger::LogHandle;
use serde::de::DeserializeOwned;

use crate::{
    config::dev_mode, container::IamContainer, error::ProxyResult, manager_api::ManagerClient,
    policy::Statement, type_alias::HttpClient,
};

pub type ArcState<S, C> = Arc<ArcSwap<ProxyState<S, C>>>;

#[async_trait]
pub trait GetNewState: Sized {
    async fn new_from_manager(manager: &ManagerClient) -> ProxyResult<Self>;
}

#[derive(Debug, Default)]
pub struct Health {
    pub state_update_failed_times: i32,
    pub state_last_successful_update_at: Option<Instant>,
}

#[derive(Debug, Default)]
pub struct ProxyState<S: Statement + Debug, C: GetNewState> {
    pub health: Health,
    pub log_handle: Option<LogHandle>,
    pub iam_container: IamContainer<S>,
    pub extended_config: C,
    pub manager_client: ManagerClient,
    pub http_client: HttpClient,
}

impl<S: Statement + DeserializeOwned + Debug + Default + Send + Sync + 'static, C: GetNewState>
    ProxyState<S, C>
{
    pub async fn new_from_manager() -> ProxyResult<Self> {
        let manager_client = ManagerClient::default();
        let state = ProxyState {
            health: Health::default(),
            log_handle: None,
            iam_container: IamContainer::new_from_manager(&manager_client).await?,
            extended_config: C::new_from_manager(&manager_client).await?,
            manager_client,
            http_client: Default::default(),
        };
        Ok(state)
    }
}

/// StateManager updating proxy state from piam manager periodically.
pub struct StateManager<S: Statement + Debug, C: GetNewState> {
    pub health_state: Health,
    pub arc_state: ArcState<S, C>,
}

impl<
        S: Statement + DeserializeOwned + Debug + Default + Send + Sync + 'static,
        C: GetNewState + Default + Send + Sync,
    > StateManager<S, C>
{
    pub async fn initialize() -> Self {
        let state = Self::get_new("initializing").await;
        StateManager {
            health_state: Default::default(),
            arc_state: Arc::new(ArcSwap::from_pointee(state)),
        }
    }

    pub async fn update_state(&self) {
        let state = Self::get_new("updating").await;
        self.arc_state.store(Arc::new(state));
    }

    async fn get_new(stage: &str) -> ProxyState<S, C> {
        let mut state: ProxyState<S, C> = ProxyState::default();
        let mut retry_interval = 10;
        let mut max_retry_time = 3600 * 24 / retry_interval;
        if dev_mode() {
            retry_interval = 1;
            max_retry_time = 1;
        }
        for i in 0..max_retry_time {
            match ProxyState::new_from_manager().await {
                Ok(s) => {
                    state = s;
                    break;
                }
                Err(e) => {
                    error!(
                        "ProxyState update failed while: {}, error: {}, retry_times: {}",
                        stage, e, i
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(retry_interval)).await;
                }
            }
        }
        state
    }
}
