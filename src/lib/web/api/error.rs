use rocket::Responder;
use serde::Serialize;

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
