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
            .get_all_users(&json!({"email": user_data["email"].as_str().unwrap()}))
            .await;

        let body = response
            .json::<PaginatedResponse<AdminUserResponse>>()
            .await;
        assert!(body.is_ok());

        let response = app.remove_user(&body.unwrap().data[0].id).await;
        assert_eq!(response.status(), StatusCode::OK);
    })
    .await;
}

#[tokio::test]
pub async fn remove_user_successfully_removes_user_from_database() {
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
        let response = app.get_all_users(&json!({"search": email})).await;
        assert_eq!(200, response.status());

        let body = response
            .json::<PaginatedResponse<AdminUserResponse>>()
            .await;
        assert!(body.is_ok());

        let user_id = &body.unwrap().data[0].id;

        let response = app.remove_user(user_id).await;
        assert_eq!(response.status(), StatusCode::OK);

        let response = app
            .get_all_users(&json!({"search": email}))
            .await
            .json::<PaginatedResponse<AdminUserResponse>>()
            .await
            .unwrap();

        println!("LENGTH {} \n\n\n\n", response.data.len());
        assert!(response.data.is_empty());
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

        let response = app.remove_user("id").await;
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

        let response = app.remove_user("id").await;
        assert_eq!(StatusCode::FORBIDDEN, response.status());
    })
    .await
}
