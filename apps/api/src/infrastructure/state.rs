use std::sync::Arc;

use crate::{application::services::SubscriptionService, config::Config};

#[derive(Clone)]
pub struct AppState {
    subscription_service: Arc<SubscriptionService>,
    config: Config,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let subscription_service = Arc::new(SubscriptionService::new(config.clone()));

        Self {
            subscription_service,
            config,
        }
    }

    pub fn subscription_service(&self) -> &SubscriptionService {
        &self.subscription_service
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
