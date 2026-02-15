use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::Response};
use tracing::error;

use crate::Error;

pub async fn error_logging(req: Request, next: Next) -> Response {
    let response = next.run(req).await;

    if let Some(err) = response.extensions().get::<Arc<Error>>() {
        error!(?err, "An unexpected error occurred");
    }

    response
}
