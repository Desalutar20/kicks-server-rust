use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Regular,
}
