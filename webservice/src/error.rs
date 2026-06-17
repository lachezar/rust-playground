use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("URL not found")]
    UrlNotFound,

    #[error("Invalid URL")]
    InvalidUrl,

    #[error("Password hashing failed")]
    PasswordHashError,

    #[error("JWT error: {0}")]
    JwtError(String),

    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::UserAlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::UrlNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidUrl => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::PasswordHashError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Password processing error".to_string(),
            ),
            AppError::JwtError(_) => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

// Error response schema for OpenAPI
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}
