use std::sync::Arc;

use kicksapi::features::auth::PASSWORD_MIN_LENGTH;
use reqwest::StatusCode;
use serde_json::json;
use tokio::task::JoinSet;

use crate::e2e::testapp::{TestApp, setup};

#[tokio::test]
pub async fn returns_200_when_request_is_valid() {
    setup(async |mut app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_verify(&data).await;

        let forgot_password_data = json!({
            "email": data["email"].as_str().unwrap()
        });

        let response = app.forgot_password(&forgot_password_data).await;
        assert_eq!(StatusCode::OK, response.status());
    })
    .await
}

#[tokio::test]
async fn returns_400_when_request_is_invalid() {
    setup(async |app: TestApp| {
        let test_cases = vec![
            ("", "empty email"),
            ("invalid email", "invalid email format"),
        ];

        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for (email, description) in test_cases {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                    "email": email,
                });

                let response = app.forgot_password(&data).await;
                assert_eq!(
                    StatusCode::BAD_REQUEST,
                    response.status(),
                    "Test case failed: {}",
                    description
                );
            });
        }

        requests.join_all().await;
    })
    .await;
}

#[tokio::test]
async fn returns_429_when_too_many_requests() {
    setup(|app: TestApp| async move {
        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for _ in 0..app.ratelimit_config.forgot_password {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                    "email": "invalid email",
                });

                let response = app.forgot_password(&data).await;
                assert_eq!(StatusCode::BAD_REQUEST, response.status());
            });
        }

        requests.join_all().await;

        let last_response = app
            .forgot_password(&json!({
                "email": "invalid email",
            }))
            .await;

        assert_eq!(StatusCode::TOO_MANY_REQUESTS, last_response.status());
    })
    .await;
}
