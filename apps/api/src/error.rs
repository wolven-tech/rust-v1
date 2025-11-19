use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
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

    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ExternalServiceError(_) => StatusCode::BAD_GATEWAY,
            Self::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let error_type = match &self {
            Self::BadRequest(_) => "Bad Request",
            Self::NotFound(_) => "Not Found",
            Self::InternalServerError(_) => "Internal Server Error",
            Self::ExternalServiceError(_) => "External Service Error",
            Self::ConfigError(_) => "Configuration Error",
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
        });

        (status_code, body).into_response()
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
