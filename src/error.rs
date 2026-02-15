use std::{collections::HashMap, sync::Arc};

use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use derive_more::From;

use redis::RedisError;
use serde::Serialize;
use serde_json::json;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Unauthorized,
    Forbidden,
    Conflict(String),
    Internal(String),
    #[from(serde_json::Error)]
    SerdeJson,
    #[from]
    JsonRejection(JsonRejection),
    #[from]
    Database(sqlx::Error),
    #[from]
    Redis(RedisError),
    #[from]
    Smtp(lettre::error::Error),
    #[from]
    Io(std::io::Error),
    DomainValidationError(Vec<String>),
    ValidationErrors(HashMap<String, Vec<String>>),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message, error) = match self {
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned(), None),
            Error::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_owned(), None),
            Error::Conflict(message) => (StatusCode::BAD_REQUEST, message, None),
            Error::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
                Some(self),
            ),
            Error::Io(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
                Some(self),
            ),
            Error::SerdeJson => (
                StatusCode::BAD_REQUEST,
                String::from("Invalid request"),
                Some(self),
            ),
            Error::JsonRejection(rejection) => {
                (StatusCode::BAD_REQUEST, rejection.body_text(), None)
            }
            Error::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
                Some(self),
            ),
            Error::Redis(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
                Some(self),
            ),
            Error::Smtp(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
                Some(self),
            ),
            Error::DomainValidationError(errors) => {
                (StatusCode::BAD_REQUEST, errors.join("\n"), None)
            }
            Error::ValidationErrors(errors) => {
                return (
                    StatusCode::BAD_REQUEST,
                    axum::Json(json!({"errors": errors})),
                )
                    .into_response();
            }
        };

        let error_response = ErrorResponse { message };

        let mut response = (status_code, axum::Json(error_response)).into_response();
        if let Some(err) = error {
            response.extensions_mut().insert(Arc::new(err));
        }

        response
    }
}
