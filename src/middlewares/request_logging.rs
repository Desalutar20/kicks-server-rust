use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tokio::time::Instant;
use tracing::{Instrument, field, info, info_span};
use uuid::Uuid;

pub async fn request_logging(req: Request, next: Next) -> Response {
    let start = Instant::now();

    let method = req.method();
    let uri = req.uri();
    let req_id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned())
        .unwrap_or(Uuid::new_v4().to_string());

    let span = info_span!("request", %method, %uri, req_id = %req_id, user = field::Empty);

    async {
            let mut resp = next.run(req).await;

            resp.headers_mut().append(
                "x-request-id",
                HeaderValue::from_str(&req_id).unwrap(),
            );

            info!(latency = ?start.elapsed().as_millis(), status = ?resp.status(), "finished processing:");

            resp
        }.instrument(span).await
}
