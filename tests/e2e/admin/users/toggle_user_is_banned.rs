use crate::e2e::testapp::{TestApp, setup};
use kicksapi::{
    PaginatedResponse,
    features::{
        admin::users::AdminUserResponse,
        auth::{PASSWORD_MIN_LENGTH, UserRole},
    },
};
use reqwest::StatusCode;
use serde_json::json;

#[tokio::test]
pub async fn returns_200_when_request_is_valid() {
    setup(async |app: TestApp| {
        let user_data = json!({
            "email": "user@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_verify(&user_data).await;

        let admin_data = json!({
            "email": "admin@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_sign_in(&admin_data, Some(UserRole::Admin))
            .await;

        let response = app
            .get_all_users(&json!({"search": user_data["email"].as_str().unwrap(),}))
            .await;

        let body = response
            .json::<PaginatedResponse<AdminUserResponse>>()
            .await;
        assert!(body.is_ok());

        let response = app.toggle_user_is_banned(&body.unwrap().data[0].id).await;
        assert_eq!(response.status(), StatusCode::OK);
    })
    .await;
}

#[tokio::test]
pub async fn toggle_user_is_banned_successfully_updates_user_is_banned() {
    setup(async |app: TestApp| {
        let user_data = json!({
            "email": "user@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_verify(&user_data).await;

        let admin_data = json!({
            "email": "admin@gmail.com",
            "password": "s".repeat(PASSWORD_MIN_LENGTH),
        });
        app.create_and_sign_in(&admin_data, Some(UserRole::Admin))
            .await;

        let email = user_data["email"].as_str().unwrap();
        let response = app.get_all_users(&json!({"search": email,})).await;
        assert_eq!(200, response.status());

        let body = response
            .json::<PaginatedResponse<AdminUserResponse>>()
            .await;
        assert!(body.is_ok());

        let user_id = &body.unwrap().data[0].id;

        let response = app.toggle_user_is_banned(user_id).await;
        assert_eq!(response.status(), StatusCode::OK);

        let user = app.get_user_by_email(email).await.unwrap();
        assert!(user.is_banned);

        let response = app.toggle_user_is_banned(user_id).await;
        assert_eq!(response.status(), StatusCode::OK);

        let user = app.get_user_by_email(email).await.unwrap();
        assert!(!user.is_banned);
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

        let response = app.toggle_user_is_banned("id").await;
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

        let response = app.toggle_user_is_banned("id").await;
        assert_eq!(StatusCode::FORBIDDEN, response.status());
    })
    .await
}
