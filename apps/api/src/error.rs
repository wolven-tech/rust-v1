//! API Error types
//!
//! Defines error types for the API using AllFrame patterns.

use allframe::reqwest;
use allframe::serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("External service error: {0}")]
    ExternalServiceError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl ApiError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn internal_server_error(msg: impl Into<String>) -> Self {
        Self::InternalServerError(msg.into())
    }

    pub fn external_service_error(msg: impl Into<String>) -> Self {
        Self::ExternalServiceError(msg.into())
    }

    pub fn config_error(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            Self::BadRequest(_) => 400,
            Self::NotFound(_) => 404,
            Self::InternalServerError(_) => 500,
            Self::ExternalServiceError(_) => 502,
            Self::ConfigError(_) => 500,
        }
    }

    /// Convert error to JSON response
    pub fn to_json(&self) -> String {
        let error_type = match self {
            Self::BadRequest(_) => "Bad Request",
            Self::NotFound(_) => "Not Found",
            Self::InternalServerError(_) => "Internal Server Error",
            Self::ExternalServiceError(_) => "External Service Error",
            Self::ConfigError(_) => "Configuration Error",
        };

        serde_json::json!({
            "error": error_type,
            "message": self.to_string()
        })
        .to_string()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self::InternalServerError(err.to_string())
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::ExternalServiceError(err.to_string())
    }
}
