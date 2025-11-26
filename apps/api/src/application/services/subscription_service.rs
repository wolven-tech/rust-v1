use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};

use crate::config::Config;
use crate::error::ApiError;

#[derive(Debug, Serialize)]
struct LoopsApiRequest {
    email: String,
    #[serde(rename = "userGroup")]
    user_group: String,
}

#[derive(Debug, Deserialize)]
struct LoopsApiResponse {
    success: Option<bool>,
    message: Option<String>,
    id: Option<String>,
}

pub struct SubscriptionService {
    http_client: reqwest::Client,
    config: Config,
}

impl SubscriptionService {
    pub fn new(config: Config) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config,
        }
    }

    #[instrument(skip(self), fields(email = %email))]
    pub async fn subscribe(
        &self,
        email: String,
        user_group: String,
    ) -> Result<(bool, Option<String>, Option<String>), ApiError> {
        info!("Processing subscription request");

        // Validate email format (basic validation)
        if !email.contains('@') {
            warn!("Invalid email format: {}", email);
            return Err(ApiError::bad_request("Invalid email format"));
        }

        let form_id = self
            .config
            .loops_form_id
            .as_ref()
            .ok_or_else(|| ApiError::config_error("Loops form ID not configured"))?;

        let url = format!("https://app.loops.so/api/newsletter-form/{}", form_id);

        info!("Calling Loops API at {}", url);

        let request_body = LoopsApiRequest {
            email: email.clone(),
            user_group,
        };

        let response = self
            .http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                warn!("Failed to call Loops API: {}", e);
                ApiError::external_service_error(format!("Failed to connect to Loops API: {}", e))
            })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| {
            ApiError::external_service_error(format!("Failed to read Loops API response: {}", e))
        })?;

        info!(
            "Loops API response status: {}, body: {}",
            status, response_text
        );

        let loops_response: LoopsApiResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                warn!("Failed to parse Loops API response: {}", e);
                ApiError::external_service_error(format!("Invalid response from Loops API: {}", e))
            })?;

        let success = loops_response.success.unwrap_or(status.is_success());

        if success {
            info!("Subscription successful for email: {}", email);
        } else {
            warn!("Subscription failed for email: {}", email);
        }

        Ok((success, loops_response.message, loops_response.id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_email() {
        let config = Config {
            port: 4400,
            host: "0.0.0.0".to_string(),
            environment: "test".to_string(),
            loops_api_key: None,
            loops_form_id: Some("test-form-id".to_string()),
        };

        let service = SubscriptionService::new(config);
        let result = service
            .subscribe("invalid-email".to_string(), "test".to_string())
            .await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid email format"));
    }

    #[test]
    fn test_service_creation() {
        let config = Config::default();
        let service = SubscriptionService::new(config);
        assert!(std::ptr::addr_of!(service).is_aligned());
    }
}
