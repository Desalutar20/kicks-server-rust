use reqwest::{Response, StatusCode};
use serde::Serialize;
use serde_json::{Value, json};

use crate::e2e::testapp::TestApp;

impl TestApp {
    pub async fn sign_up<Body>(&self, body: &Body) -> Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}{}", self.address, "/auth/sign-up"))
            .json(&body)
            .send()
            .await
            .expect("Request failed")
    }

    pub async fn verify_account<Body>(&self, body: &Body) -> Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}{}", self.address, "/auth/verify-account"))
            .json(&body)
            .send()
            .await
            .expect("Request failed")
    }

    pub async fn sign_in<Body>(&self, body: &Body) -> Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}{}", self.address, "/auth/sign-in"))
            .json(&body)
            .send()
            .await
            .expect("Request failed")
    }

    pub async fn forgot_password<Body>(&self, body: &Body) -> Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}{}", self.address, "/auth/forgot-password"))
            .json(&body)
            .send()
            .await
            .expect("Request failed")
    }

    pub async fn reset_password<Body>(&self, body: &Body) -> Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(format!("{}{}", self.address, "/auth/reset-password"))
            .json(&body)
            .send()
            .await
            .expect("Request failed")
    }

    pub async fn logout(&self) -> Response {
        self.http_client
            .post(format!("{}{}", self.address, "/auth/logout"))
            .send()
            .await
            .expect("Request failed")
    }

    pub async fn get_me(&self) -> Response {
        self.http_client
            .get(format!("{}{}", self.address, "/auth/me"))
            .send()
            .await
            .expect("Request failed")
    }

    pub async fn create_and_verify(&mut self, body: &Value) {
        let response = self.sign_up(body).await;
        assert_eq!(StatusCode::CREATED, response.status());

        let token = self
            .get_redis_value(crate::e2e::testapp::RedisKeyType::AccountVerification)
            .await;

        assert!(token.is_some());

        let verify_account_data = json!({
            "email": body["email"].as_str().unwrap(),
            "token": token.unwrap(),
        });

        let verify_response = self.verify_account(&verify_account_data).await;
        assert_eq!(StatusCode::OK, verify_response.status());
    }

    pub async fn create_and_sign_in(&mut self, body: &Value) {
        self.create_and_verify(body).await;

        let signin_data = json!({
            "email": body["email"].as_str().unwrap(),
            "password": body["password"].as_str().unwrap(),
        });

        let response = self.sign_in(&signin_data).await;
        assert_eq!(StatusCode::OK, response.status());

        let cookie = response
            .cookies()
            .find(|c| c.name() == self.application_config.session_cookie_name);

        assert!(cookie.is_some());
    }
}
