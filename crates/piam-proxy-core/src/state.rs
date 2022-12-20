use std::{fmt::Debug, sync::Arc, time::Instant};

use arc_swap::ArcSwap;
use async_trait::async_trait;
use log::warn;
use piam_common::logger::LogHandle;
use serde::de::DeserializeOwned;

use crate::{
    config::dev_mode,
    container::IamContainer,
    error::{eok, ProxyResult},
    manager_api::ManagerClient,
    policy::Modeled,
    type_alias::HttpClient,
};

pub type ArcState<P, C> = Arc<ArcSwap<ProxyState<P, C>>>;

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
pub struct ProxyState<P: Modeled, C: GetNewState> {
    pub health: Health,
    pub log_handle: Option<LogHandle>,
    pub iam_container: IamContainer<P>,
    pub extended_config: C,
    pub manager_client: ManagerClient,
    pub http_client: HttpClient,
}

impl<P: Modeled + DeserializeOwned + Default + Send + Sync + 'static, C: GetNewState>
    ProxyState<P, C>
{
    pub async fn new_from_manager() -> ProxyResult<Self> {
        let manager_client = ManagerClient::default();
        // TODO: do new_from_manager stuff here make sub new_from_manager state independent
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
pub struct StateManager<P: Modeled, C: GetNewState> {
    pub health_state: Health,
    pub arc_state: ArcState<P, C>,
}

impl<
        P: Modeled + DeserializeOwned + Default + Send + Sync + 'static,
        C: GetNewState + Default + Send + Sync,
    > StateManager<P, C>
{
    pub async fn initialize() -> Self {
        let get_result: ProxyResult<ProxyState<P, C>> = Self::get_new(When::Initializing).await;
        let state = eok(get_result);
        StateManager {
            health_state: Default::default(),
            arc_state: Arc::new(ArcSwap::from_pointee(state)),
        }
    }

    pub async fn update_state(&self) {
        let get_result: ProxyResult<ProxyState<P, C>> = Self::get_new(When::Updating).await;
        match get_result {
            Ok(s) => self.arc_state.store(Arc::new(s)),
            Err(e) => warn!("ProxyState updating failed, error: {}", e),
        };
    }

    async fn get_new(when: When) -> ProxyResult<ProxyState<P, C>> {
        let retry_interval = 5;
        let mut retry_count = 0;
        match when {
            When::Initializing => loop {
                match ProxyState::new_from_manager().await {
                    Ok(state) => {
                        return Ok(state);
                    }
                    Err(e) => {
                        warn!(
                            "ProxyState {} failed, error: {}, retry_count: {}",
                            when, e, retry_count
                        );
                        if dev_mode() && retry_count > 1 {
                            tokio::time::sleep(std::time::Duration::from_secs(retry_count * 5))
                                .await;
                        }
                        tokio::time::sleep(std::time::Duration::from_secs(retry_interval)).await;
                        retry_count += 1;
                    }
                }
            },
            When::Updating => ProxyState::new_from_manager().await,
        }
    }
}

enum When {
    Initializing,
    Updating,
}

impl std::fmt::Display for When {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            When::Initializing => write!(f, "initialization"),
            When::Updating => write!(f, "updating"),
        }
    }
}
