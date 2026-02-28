use reqwest::Response;
use serde::Serialize;

use crate::e2e::testapp::TestApp;

impl TestApp {
    pub async fn get_all_users<Query: Serialize>(&self, query: &Query) -> Response {
        self.http_client
            .get(format!("{}{}", self.address, "/admin/users"))
            .query(query)
            .send()
            .await
            .expect("Request failed")
    }

    pub async fn toggle_user_is_banned(&self, user_id: &str) -> Response {
        self.http_client
            .post(format!(
                "{}{}/{}{}",
                self.address, "/admin/users", user_id, "/toggle-is-banned"
            ))
            .send()
            .await
            .expect("Request failed")
    }

    pub async fn remove_user(&self, user_id: &str) -> Response {
        self.http_client
            .delete(format!("{}{}/{}", self.address, "/admin/users", user_id,))
            .send()
            .await
            .expect("Request failed")
    }
}
