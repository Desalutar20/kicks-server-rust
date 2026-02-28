use std::sync::Arc;

use kicksapi::{
    PaginatedResponse,
    features::{
        admin::users::{
            AdminUserResponse, GET_ALL_USERS_MAX_LIMIT, GET_ALL_USERS_SEARCH_MAX_LENGTH,
        },
        auth::{PASSWORD_MIN_LENGTH, UserRole},
    },
};
use reqwest::StatusCode;
use serde_json::json;
use tokio::task::JoinSet;

use crate::e2e::testapp::{TestApp, setup};

#[tokio::test]
pub async fn returns_200_when_request_is_valid() {
    setup(async |app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_sign_in(&data, Some(UserRole::Admin)).await;

        let response = app.get_all_users(&json!({})).await;

        let body = response
            .json::<PaginatedResponse<AdminUserResponse>>()
            .await;
        assert!(body.is_ok());
    })
    .await
}

#[tokio::test]
async fn returns_400_when_request_is_invalid() {
    setup(async |app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_sign_in(&data, Some(UserRole::Admin)).await;

        let test_cases = vec![
            (json!({"is_banned": "not boolean"}), "invalid is_banned"),
            (json!({"is_verified": "not boolean"}), "invalid is_verified"),
            (json!({"start_date": "not date"}), "invalid start_date"),
            (json!({"end_date": "not date"}), "invalid end_date"),
            (json!({"search": ""}), "empty search"),
            (
                json!({"search": "s".repeat(GET_ALL_USERS_SEARCH_MAX_LENGTH + 1)}),
                "search too long",
            ),
            (json!({"gender": "invalid gender"}), "invalid gender"),
            (json!({"page": "not number"}), "invalid page"),
            (json!({"page": 0}), "page is zero"),
            (json!({"limit": "not number"}), "invalid limit"),
            (json!({"limit": 0}), "limit is zero"),
            (
                json!({"limit": GET_ALL_USERS_MAX_LIMIT + 1}),
                "limit exceeds maximum",
            ),
        ];

        let mut requests = JoinSet::new();
        let app = Arc::new(app);

        for (query, description) in test_cases {
            let app = app.clone();
            requests.spawn(async move {
                let response = app.get_all_users(&query).await;
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
pub async fn returns_401_when_user_is_not_authorized() {
    setup(async |app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_verify(&data).await;

        let response = app.get_all_users(&json!({})).await;
        assert_eq!(StatusCode::UNAUTHORIZED, response.status());
    })
    .await
}

#[tokio::test]
pub async fn returns_403_when_user_is_not_admin() {
    setup(async |app: TestApp| {
        let data = json!({
            "email": "test@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });

        app.create_and_sign_in(&data, None).await;

        let response = app.get_all_users(&json!({})).await;
        assert_eq!(StatusCode::FORBIDDEN, response.status());
    })
    .await
}
