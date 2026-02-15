use std::sync::Arc;

use kicksapi::features::auth::{PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH};
use reqwest::StatusCode;
use serde_json::json;
use tokio::task::JoinSet;

use crate::e2e::testapp::{RedisKeyType, TestApp, setup};

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

        let forgot_password_response = app.forgot_password(&forgot_password_data).await;
        assert_eq!(StatusCode::OK, forgot_password_response.status());

        let token = app.get_redis_value(RedisKeyType::ResetPassword).await;
        assert!(token.is_some());

        let reset_password_data = json!({
            "email":  data["email"].as_str().unwrap(),
            "token": token.unwrap().to_string(),
            "new_password": "n".repeat(PASSWORD_MIN_LENGTH),
        });

        let response = app.reset_password(&reset_password_data).await;
        assert_eq!(StatusCode::OK, response.status());
    })
    .await
}

#[tokio::test]
pub async fn reset_password_successfully_updates_user_password() {
    setup(async |mut app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_verify(&data).await;

        let forgot_password_data = json!({
            "email": data["email"].as_str().unwrap()
        });

        let forgot_password_response = app.forgot_password(&forgot_password_data).await;
        assert_eq!(StatusCode::OK, forgot_password_response.status());

        let token = app.get_redis_value(RedisKeyType::ResetPassword).await;
        assert!(token.is_some());

        let new_password = "n".repeat(PASSWORD_MIN_LENGTH);

        let reset_password_data = json!({
            "email":  data["email"].as_str().unwrap(),
            "token": token.unwrap().to_string(),
            "new_password": new_password,
        });

        let user_before = app.get_user_by_email(data["email"].as_str().unwrap()).await;
        assert!(user_before.is_some());

        let response = app.reset_password(&reset_password_data).await;
        assert_eq!(StatusCode::OK, response.status());

        let user_after = app.get_user_by_email(data["email"].as_str().unwrap()).await;

        assert!(user_after.is_some());
        assert_ne!(user_before.unwrap().password, user_after.unwrap().password);
    })
    .await
}

#[tokio::test]
async fn returns_400_when_request_is_invalid() {
    setup(async |app: TestApp| {
        let test_cases = vec![
            (
                "",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "some_token",
                "empty email",
            ),
            (
                "invalid email",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "some_token",
                "invalid email format",
            ),
            ("test@gmail.com", "".into(), "some_token", "empty password"),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH - 1),
                "some_token",
                "password too short",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MAX_LENGTH + 1),
                "some_token",
                "password too long",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "",
                "empty token",
            ),
        ];

        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for (email, new_password, token, description) in test_cases {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                "email": email,
                "new_password": new_password,
                "token": token,
                });

                let response = app.reset_password(&data).await;
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
        app.create_and_verify(&data).await;

        let forgot_password_data = json!({
            "email": data["email"].as_str().unwrap()
        });

        let forgot_password_response = app.forgot_password(&forgot_password_data).await;
        assert_eq!(StatusCode::OK, forgot_password_response.status());

        let token = app.get_redis_value(RedisKeyType::ResetPassword).await;
        assert!(token.is_some());

        app.ban_user(data["email"].as_str().unwrap()).await;

        let reset_password_data = json!({
            "email":  data["email"].as_str().unwrap(),
            "token": token.unwrap().to_string(),
            "new_password": "n".repeat(PASSWORD_MIN_LENGTH),
        });

        let response = app.reset_password(&reset_password_data).await;
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    })
    .await
}

#[tokio::test]
async fn returns_400_when_email_is_different() {
    setup(async |mut app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_verify(&data).await;

        let forgot_password_data = json!({
            "email": data["email"].as_str().unwrap()
        });

        let forgot_password_response = app.forgot_password(&forgot_password_data).await;
        assert_eq!(StatusCode::OK, forgot_password_response.status());

        let token = app.get_redis_value(RedisKeyType::ResetPassword).await;
        assert!(token.is_some());

        let reset_password_data = json!({
            "email": "random@gmail.com",
            "token": token.unwrap().to_string(),
            "new_password": "n".repeat(PASSWORD_MIN_LENGTH),
        });

        let response = app.reset_password(&reset_password_data).await;
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    })
    .await;
}

#[tokio::test]
async fn returns_429_when_too_many_requests() {
    setup(|app: TestApp| async move {
        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for _ in 0..app.ratelimit_config.reset_password {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                    "email": "invalid email",
                    "new_password": "password",
                    "token": "token",
                });
                let response = app.reset_password(&data).await;
                assert_eq!(StatusCode::BAD_REQUEST, response.status());
            });
        }

        requests.join_all().await;

        let last_response = app
            .reset_password(&json!({
                "email": "invalid email",
                "new_password": "password",
                "token": "token",
            }))
            .await;

        assert_eq!(StatusCode::TOO_MANY_REQUESTS, last_response.status());
    })
    .await;
}
