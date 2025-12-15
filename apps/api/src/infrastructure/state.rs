use std::sync::Arc;

use crate::{
    application::services::{AllFrameService, SubscriptionService},
    config::Config,
};

#[derive(Clone)]
pub struct AppState {
    subscription_service: Arc<SubscriptionService>,
    allframe_service: Arc<AllFrameService>,
    config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let subscription_service = Arc::new(SubscriptionService::new(config.clone()));
        let allframe_service = Arc::new(AllFrameService::new());

        Self {
            subscription_service,
            allframe_service,
            config,
        }
    }

    pub fn subscription_service(&self) -> &SubscriptionService {
        &self.subscription_service
    }

    pub fn allframe_service(&self) -> &AllFrameService {
        &self.allframe_service
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
