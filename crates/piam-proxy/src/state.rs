use std::{fmt::Debug, sync::Arc, time::Instant};

use arc_swap::ArcSwap;
use async_trait::async_trait;
use busylib::{config::dev_mode, logger::LogHandle, prelude::EnhancedUnwrap};
use log::{debug, warn};
use piam_core::policy::Modeled;
use serde::de::DeserializeOwned;

use crate::{
    config::{CoreConfig, EXTENDED_CONFIG_TYPE},
    container::IamContainer,
    error::ProxyResult,
    manager_api::ManagerClient,
    type_alias::HttpClient,
};

pub type ArcState<P, C> = Arc<ArcSwap<ProxyState<P, C>>>;

pub trait CoreState<C>: Sized {
    fn new_from(config: C) -> ProxyResult<Self>;
}

#[async_trait]
pub trait ExtendedState<C, P: Modeled>: Sized {
    fn new_from(extended_config: C) -> ProxyResult<Self>;
    async fn with_core_config(self, core_config: &CoreConfig<P>) -> ProxyResult<Self>;
}

#[derive(Debug, Default)]
pub struct Health {
    pub state_update_failed_times: i32,
    pub state_last_successful_update_at: Option<Instant>,
}

#[derive(Debug, Default)]
pub struct ProxyState<P: Modeled, C: ExtendedState<C, P>> {
    pub health: Health,
    pub log_handle: Option<LogHandle>,
    pub iam_container: IamContainer<P>,
    pub extended_config: C,
    pub manager_client: ManagerClient,
    pub http_client: HttpClient,
}

impl<
        P: Modeled + DeserializeOwned + Default + Send + Sync + 'static,
        C: ExtendedState<C, P> + DeserializeOwned,
    > ProxyState<P, C>
{
    pub async fn new_from_manager() -> ProxyResult<Self> {
        let manager_client = ManagerClient::default();
        if !dev_mode() {
            debug!("start fetching config");
        }
        let core_config = manager_client.get_core_config().await?;
        let extended_config = manager_client
            .get_extended_config(&EXTENDED_CONFIG_TYPE.load())
            .await?;
        if !dev_mode() {
            debug!("end fetching config");
        }

        let extended_config = C::new_from(extended_config)?
            .with_core_config(&core_config)
            .await?;

        let iam_container = IamContainer::new_from(core_config)?;

        let state = Self {
            health: Health::default(),
            log_handle: None,
            iam_container,
            extended_config,
            manager_client,
            http_client: Default::default(),
        };
        Ok(state)
    }
}

/// StateManager updating proxy state from piam manager periodically.
pub struct StateManager<P: Modeled, C: ExtendedState<C, P>> {
    pub health_state: Health,
    pub arc_state: ArcState<P, C>,
}

impl<
        P: Modeled + DeserializeOwned + Default + Send + Sync + 'static,
        C: ExtendedState<C, P> + Debug + DeserializeOwned + Default + Send + Sync,
    > StateManager<P, C>
{
    pub async fn initialize() -> Self {
        let state = Self::get_new(When::Initializing).await.unwp();
        Self {
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
        let mut retries = 0;
        match when {
            When::Initializing => loop {
                match ProxyState::new_from_manager().await {
                    Ok(state) => {
                        return Ok(state);
                    }
                    Err(e) => {
                        warn!(
                            "ProxyState {} failed, error: {}, retries: {}",
                            when, e, retries
                        );
                        if dev_mode() && retries > 1 {
                            tokio::time::sleep(std::time::Duration::from_secs(retries * 5)).await;
                        }
                        tokio::time::sleep(std::time::Duration::from_secs(retry_interval)).await;
                        retries += 1;
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
            Self::Initializing => write!(f, "initialization"),
            Self::Updating => write!(f, "updating"),
        }
    }
}
