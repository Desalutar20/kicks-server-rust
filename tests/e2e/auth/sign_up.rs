use std::sync::Arc;

use kicksapi::features::auth::{
    FIRST_NAME_MAX_LENGTH, LAST_NAME_MAX_LENGTH, PASSWORD_MAX_LENGTH, PASSWORD_MIN_LENGTH,
};
use reqwest::StatusCode;
use serde_json::json;
use tokio::task::JoinSet;

use crate::e2e::testapp::{TestApp, setup};

#[tokio::test]
async fn returns_201_when_request_is_valid() {
    setup(async |app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });

        let response = app.sign_up(&data).await;
        assert_eq!(StatusCode::CREATED, response.status());
    })
    .await;
}

#[tokio::test]
async fn saves_user_into_database_when_request_is_valid() {
    setup(async |app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });

        let response = app.sign_up(&data).await;
        assert_eq!(StatusCode::CREATED, response.status());

        let user = app.get_user_by_email(data["email"].as_str().unwrap()).await;

        assert!(user.is_some());
        assert_ne!(
            data["password"].as_str().unwrap(),
            user.unwrap().password.unwrap().as_ref()
        );
    })
    .await;
}

#[tokio::test]
async fn returns_400_when_request_is_invalid() {
    setup(async |app: TestApp| {
        let test_cases = vec![
            (
                "",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "f".into(),
                "l".into(),
                None,
                "empty email",
            ),
            (
                "invalid email",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "f".into(),
                "l".into(),
                None,
                "invalid email format",
            ),
            (
                "test@gmail.com",
                "".to_string(),
                "f".into(),
                "l".into(),
                None,
                "empty password",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH - 1),
                "f".into(),
                "l".into(),
                None,
                "password too short",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MAX_LENGTH + 1),
                "f".into(),
                "l".into(),
                None,
                "password too long",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "".into(),
                "l".into(),
                None,
                "empty first_name",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "f".repeat(FIRST_NAME_MAX_LENGTH + 1),
                "l".into(),
                None,
                "first_name too long",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "John123".to_string(),
                "Doe".into(),
                None,
                "first_name contains non-alphabetic characters",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "f".into(),
                "".into(),
                None,
                "empty last_name",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "f".into(),
                "l".repeat(LAST_NAME_MAX_LENGTH + 1),
                None,
                "last_name too long",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "John".into(),
                "Doe_42".into(),
                None,
                "last_name contains non-alphabetic characters",
            ),
            (
                "test@gmail.com",
                "p".repeat(PASSWORD_MIN_LENGTH),
                "f".into(),
                "l".into(),
                Some("invalid gender"),
                "invalid gender",
            ),
        ];

        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for (email, password, first_name, last_name, gender, description) in test_cases {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                    "email": email,
                    "password": password,
                    "first_name": first_name,
                    "last_name": last_name,
                    "gender": gender,
                });

                let response = app.sign_up(&data).await;
                let status = response.status();
                let text = response.text().await;

                if status == StatusCode::INTERNAL_SERVER_ERROR {
                    println!("{:#?}", text);
                }

                assert_eq!(
                    StatusCode::BAD_REQUEST,
                    status,
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
async fn returns_400_when_user_already_exists() {
    setup(async |app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        let response = app.sign_up(&data).await;
        assert_eq!(StatusCode::CREATED, response.status());

        let second_response = app.sign_up(&data).await;
        assert_eq!(StatusCode::BAD_REQUEST, second_response.status());
    })
    .await;
}

#[tokio::test]
async fn returns_429_when_too_many_requests() {
    setup(|app: TestApp| async move {
        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for _ in 0..app.ratelimit_config.sign_up {
            let app = app.clone();
            requests.spawn(async move {
                let data = json!({
                    "email": "invalid email",
                    "password": "password",
                });
                let response = app.sign_up(&data).await;
                assert_eq!(StatusCode::BAD_REQUEST, response.status());
            });
        }

        requests.join_all().await;

        let data = json!({
            "email": "invalid email",
            "password": "password",
        });
        let last_response = app.sign_up(&data).await;

        assert_eq!(StatusCode::TOO_MANY_REQUESTS, last_response.status());
    })
    .await;
}
