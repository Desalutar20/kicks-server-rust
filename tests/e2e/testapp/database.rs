use kicksapi::features::auth::{
    EmailAddress, FacebookID, FirstName, GoogleID, HashedPassword, LastName,
    REDIS_ACCOUNT_VERIFICATION_PREFIX, REDIS_RESET_PASSWORD_PREFIX, User, UserGender, UserID,
    UserRole,
};
use redis::AsyncTypedCommands;
use sqlx::query;

use crate::e2e::testapp::TestApp;

pub enum RedisKeyType {
    AccountVerification,
    ResetPassword,
}

impl TestApp {
    pub async fn get_user_by_email(&self, email: &str) -> Option<User> {
        let record = query!(
            r#"
                SELECT
                    id,
                    created_at,
                    updated_at,
                    email,
                    password,
                    first_name,
                    last_name,
                    role as "role: UserRole",
                    gender as "gender: UserGender",
                    is_verified,
                    is_banned,
                    google_id,
                    facebook_id
                FROM users
                WHERE email = $1
                "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .expect("");

        if let Some(record) = record {
            let user = User {
                id: UserID::from(record.id),
                created_at: record.created_at,
                updated_at: record.updated_at,
                email: EmailAddress::parse(record.email).unwrap(),
                password: record
                    .password
                    .map(HashedPassword::parse)
                    .transpose()
                    .unwrap(),
                first_name: record.first_name.map(FirstName::parse).transpose().unwrap(),
                last_name: record.last_name.map(LastName::parse).transpose().unwrap(),
                role: record.role,
                gender: record.gender,
                is_verified: record.is_verified,
                is_banned: record.is_banned,
                google_id: record.google_id.map(GoogleID::parse).transpose().unwrap(),
                facebook_id: record
                    .facebook_id
                    .map(FacebookID::parse)
                    .transpose()
                    .unwrap(),
            };

            Some(user)
        } else {
            None
        }
    }

    pub async fn ban_user(&self, email: &str) {
        sqlx::query!(
            r#"
        UPDATE users SET is_banned = true
        WHERE email = $1;
        "#,
            email
        )
        .execute(&self.pool)
        .await
        .expect("Failed to ban user");
    }

    pub async fn get_redis_value(&mut self, key_type: RedisKeyType) -> Option<String> {
        let (pattern, prefix) = match key_type {
            RedisKeyType::AccountVerification => (
                format!("{}*", REDIS_ACCOUNT_VERIFICATION_PREFIX),
                REDIS_ACCOUNT_VERIFICATION_PREFIX,
            ),
            RedisKeyType::ResetPassword => (
                format!("{}*", REDIS_RESET_PASSWORD_PREFIX),
                REDIS_RESET_PASSWORD_PREFIX,
            ),
        };

        let verification_keys = self
            .redis
            .keys(pattern)
            .await
            .expect("Failed to get redis keys");

        verification_keys
            .last()
            .cloned()
            .and_then(|k| k.strip_prefix(prefix).map(|s| s.to_string()))
    }
}
