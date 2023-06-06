use rocket::{serde::json::Json, Responder};
use serde::Serialize;

use crate::ServiceError;

#[derive(Responder, Debug, thiserror::Error, Serialize)]
pub enum ApiKeyError {
    /// API key not found.
    #[error("API key not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(String),
    /// Invalid API key format.
    #[error("invalid API key format")]
    #[response(status = 400, content_type = "json")]
    DecodeError(String),
}

#[derive(Responder, Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),

    #[error("server error")]
    #[response(status = 500, content_type = "json")]
    Server(Json<String>),

    #[error("client error")]
    #[response(status = 401, content_type = "json")]
    User(Json<String>),

    #[error("key error")]
    #[response(status = 400, content_type = "json")]
    KeyError(Json<ApiKeyError>),
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Clip(c) => Self::User(Json(format!("clip parsing error: {}", c))),
            ServiceError::NotFound => Self::NotFound(Json("entity not found".to_owned())),
            ServiceError::Data(_) => Self::Server(Json("a server error occurred".to_owned())),
            ServiceError::PermissionError(msg) => Self::User(Json(msg)),
        }
    }
}
