use std::sync::Arc;

use kicksapi::features::auth::PASSWORD_MIN_LENGTH;
use reqwest::StatusCode;
use serde_json::json;
use tokio::task::JoinSet;

use crate::e2e::testapp::{RedisKeyType, TestApp, setup};

#[tokio::test]
async fn returns_200_when_request_is_valid() {
    setup(async |mut app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });

        let response = app.sign_up(&data).await;
        assert_eq!(StatusCode::CREATED, response.status());

        let token = app.get_redis_value(RedisKeyType::AccountVerification).await;

        assert!(token.is_some());

        let verify_account_data = json!({
            "email": data["email"].as_str().unwrap(),
            "token": token.unwrap(),
        });

        let verification_response = app.verify_account(&verify_account_data).await;
        assert_eq!(StatusCode::OK, verification_response.status());
    })
    .await;
}

#[tokio::test]
async fn user_should_be_verified_when_request_is_valid() {
    setup(async |mut app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });

        let response = app.sign_up(&data).await;
        assert_eq!(StatusCode::CREATED, response.status());

        let token = app.get_redis_value(RedisKeyType::AccountVerification).await;

        assert!(token.is_some());

        let verify_account_data = json!({
            "email": data["email"].as_str().unwrap(),
            "token": token.unwrap(),
        });

        let verification_response = app.verify_account(&verify_account_data).await;
        assert_eq!(StatusCode::OK, verification_response.status());

        let user = app.get_user_by_email(data["email"].as_str().unwrap()).await;
        assert!(user.is_some());
        assert!(user.unwrap().is_verified);
    })
    .await;
}

#[tokio::test]
async fn returns_400_when_request_is_invalid() {
    setup(async |app: TestApp| {
        let test_cases = vec![
            ("", "token", "empty email"),
            ("invalid email", "token", "invalid email format"),
            ("test@gmail.com", "", "empty token"),
        ];

        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for (email, token, description) in test_cases {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                    "email": email,
                    "token": token,
                });

                let response = app.verify_account(&data).await;
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
async fn returns_400_when_user_is_banned() {
    setup(async |mut app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });

        let response = app.sign_up(&data).await;
        assert_eq!(StatusCode::CREATED, response.status());

        let token = app.get_redis_value(RedisKeyType::AccountVerification).await;
        assert!(token.is_some());

        app.ban_user(data["email"].as_str().unwrap()).await;

        let verify_account_data = json!({
            "email": data["email"].as_str().unwrap(),
            "token": token.unwrap(),
        });

        let verification_response = app.verify_account(&verify_account_data).await;
        assert_eq!(StatusCode::BAD_REQUEST, verification_response.status());
    })
    .await;
}

#[tokio::test]
async fn returns_400_when_email_is_different() {
    setup(async |mut app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });

        let response = app.sign_up(&data).await;
        assert_eq!(StatusCode::CREATED, response.status());

        let token = app.get_redis_value(RedisKeyType::AccountVerification).await;
        assert!(token.is_some());

        let verify_account_data = json!({
            "email": "random@gmail.com",
            "token": token.unwrap(),
        });

        let verification_response = app.verify_account(&verify_account_data).await;
        assert_eq!(StatusCode::BAD_REQUEST, verification_response.status());
    })
    .await;
}

#[tokio::test]
async fn returns_429_when_too_many_requests() {
    setup(|app: TestApp| async move {
        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for _ in 0..app.ratelimit_config.verify_account {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                    "email": "invalid email",
                    "token": "token",
                });
                let response = app.verify_account(&data).await;
                assert_eq!(StatusCode::BAD_REQUEST, response.status());
            });
        }

        requests.join_all().await;

        let last_response = app
            .verify_account(&json!({
                "email": "invalid email",
                "token": "token",
            }))
            .await;

        assert_eq!(StatusCode::TOO_MANY_REQUESTS, last_response.status());
    })
    .await;
}
