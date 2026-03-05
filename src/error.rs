use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};
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
    DomainValidationError(Vec<String>),
    ValidationErrors(HashMap<String, Vec<String>>),
    #[from(serde_json::Error)]
    SerdeJson,
    #[from]
    JsonRejection(JsonRejection),
    #[from]
    QueryRejection(QueryRejection),
    #[from]
    Database(sqlx::Error),
    #[from]
    Redis(RedisError),
    #[from]
    Smtp(lettre::error::Error),
    #[from]
    Io(std::io::Error),
    #[from]
    Reqwest(reqwest::Error),
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status_code, message, error) = match self {
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_owned(), None),
            Error::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_owned(), None),
            Error::Conflict(message) => (StatusCode::BAD_REQUEST, message, None),
            Error::SerdeJson => (
                StatusCode::BAD_REQUEST,
                String::from("Invalid request"),
                Some(self),
            ),
            Error::JsonRejection(rejection) => {
                (StatusCode::BAD_REQUEST, rejection.body_text(), None)
            }
            Error::QueryRejection(rejection) => {
                (StatusCode::BAD_REQUEST, rejection.body_text(), None)
            }
            Error::Database(_)
            | Error::Redis(_)
            | Error::Smtp(_)
            | Error::Reqwest(_)
            | Error::Internal(_)
            | Error::Io(_) => (
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

        let error_response = ErrorResponse { error: message };

        let mut response = (status_code, axum::Json(error_response)).into_response();
        if let Some(err) = error {
            response.extensions_mut().insert(Arc::new(err));
        }

        response
    }
}
