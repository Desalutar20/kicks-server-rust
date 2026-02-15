use std::sync::Arc;

use kicksapi::{
    ApiResponse,
    features::auth::{PASSWORD_MIN_LENGTH, UserResponse},
};
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

        app.create_and_sign_in(&data).await;

        let response = app.get_me().await;
        assert_eq!(StatusCode::OK, response.status());

        let body = response.json::<ApiResponse<UserResponse>>().await;
        assert!(body.is_ok());
    })
    .await
}

#[tokio::test]
pub async fn returns_401_when_user_is_not_authorized() {
    setup(async |app: TestApp| {
        let response = app.get_me().await;
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    })
    .await
}

#[tokio::test]
pub async fn returns_401_when_user_is_banned() {
    setup(async |mut app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_sign_in(&data).await;

        app.ban_user(data["email"].as_str().unwrap()).await;

        let response = app.get_me().await;
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    })
    .await
}

#[tokio::test]
async fn returns_429_when_too_many_requests() {
    setup(|app: TestApp| async move {
        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for _ in 0..app.ratelimit_config.get_me {
            let app = app.clone();
            requests.spawn(async move {
                let response = app.get_me().await;
                assert_eq!(StatusCode::UNAUTHORIZED, response.status());
            });
        }

        requests.join_all().await;

        let last_response = app.get_me().await;
        assert_eq!(StatusCode::TOO_MANY_REQUESTS, last_response.status());
    })
    .await;
}
