use std::sync::Arc;

use kicksapi::{
    ApiResponse,
    features::auth::{PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH, UserResponse},
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
        app.create_and_verify(&data).await;

        let signin_data = json!({
            "email": data["email"].as_str().unwrap(),
            "password":  data["password"].as_str().unwrap(),
        });

        let response = app.sign_in(&signin_data).await;
        assert_eq!(StatusCode::OK, response.status());

        let body = response.json::<ApiResponse<UserResponse>>().await;
        assert!(body.is_ok());
    })
    .await
}

#[tokio::test]
pub async fn successful_sign_in_returns_http_only_cookie() {
    setup(async |mut app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_verify(&data).await;

        let signin_data = json!({
            "email": data["email"].as_str().unwrap(),
            "password":  data["password"].as_str().unwrap(),
        });

        let response = app.sign_in(&signin_data).await;
        assert_eq!(StatusCode::OK, response.status());

        let cookie = response
            .cookies()
            .find(|c| c.name() == app.application_config.session_cookie_name);

        assert!(cookie.is_some());
        assert!(cookie.unwrap().http_only());
    })
    .await
}

#[tokio::test]
async fn returns_400_when_request_is_invalid() {
    setup(async |app: TestApp| {
        let test_cases = vec![
            ("", "p".repeat(PASSWORD_MIN_LENGTH), "empty email"),
            (
                "invalid email",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "invalid email format",
            ),
            ("test@gmail.com", "".to_string(), "empty password"),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH - 1),
                "password too short",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MAX_LENGTH + 1),
                "password too long",
            ),
        ];

        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for (email, password, description) in test_cases {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                    "email": email,
                    "password": password
                });

                let response = app.sign_in(&data).await;
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

        app.ban_user(data["email"].as_str().unwrap()).await;

        let signin_data = json!({
            "email": data["email"].as_str().unwrap(),
            "password":  data["password"].as_str().unwrap(),
        });

        let response = app.sign_in(&signin_data).await;
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    })
    .await
}

#[tokio::test]
async fn returns_400_when_user_not_verified() {
    setup(async |app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        let response = app.sign_up(&data).await;

        assert_eq!(StatusCode::CREATED, response.status());

        let signin_data = json!({
            "email": data["email"].as_str().unwrap(),
            "password":  data["password"].as_str().unwrap(),
        });

        let response = app.sign_in(&signin_data).await;
        assert_eq!(StatusCode::BAD_REQUEST, response.status());
    })
    .await
}

#[tokio::test]
async fn returns_429_when_too_many_requests() {
    setup(|app: TestApp| async move {
        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for _ in 0..app.ratelimit_config.sign_in {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                    "email": "invalid@gmail.com",
                    "password": "password"
                });
                let response = app.sign_in(&data).await;
                assert_eq!(StatusCode::BAD_REQUEST, response.status());
            });
        }

        requests.join_all().await;

        let last_response = app
            .sign_in(&json!({
                "email": "invalid@gmail.com",
                "password": "password"
            }))
            .await;

        assert_eq!(StatusCode::TOO_MANY_REQUESTS, last_response.status());
    })
    .await;
}
