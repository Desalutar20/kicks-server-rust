use sqlx::{PgPool, query};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    Result,
    features::auth::{
        EmailAddress, FacebookID, FirstName, GoogleID, HashedPassword, LastName,
        domain::{NewUser, UpdateUser, User, UserGender, UserID, UserRole},
    },
};
#[derive(Debug, Clone)]
pub struct AuthRepository {
    pool: PgPool,
}

impl AuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip_all, name = "authrepository - get user by email")]
    pub async fn get_user_by_email(&self, email: &EmailAddress) -> Result<Option<User>> {
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
            email.as_ref()
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(record) = record {
            let user = User {
                id: UserID::from(record.id),
                created_at: record.created_at,
                updated_at: record.updated_at,
                email: EmailAddress::parse(record.email)?,
                password: record.password.map(HashedPassword::parse).transpose()?,
                first_name: record.first_name.map(FirstName::parse).transpose()?,
                last_name: record.last_name.map(LastName::parse).transpose()?,
                role: record.role,
                gender: record.gender,
                is_verified: record.is_verified,
                is_banned: record.is_banned,
                google_id: record.google_id.map(GoogleID::parse).transpose()?,
                facebook_id: record.facebook_id.map(FacebookID::parse).transpose()?,
            };

            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    #[instrument(skip_all, name = "authrepository - get user by id")]
    pub async fn get_user_by_id(&self, id: &UserID) -> Result<Option<User>> {
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
                WHERE id = $1
                "#,
            id.as_ref()
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(record) = record {
            let user = User {
                id: UserID::from(record.id),
                created_at: record.created_at,
                updated_at: record.updated_at,
                email: EmailAddress::parse(record.email)?,
                password: record.password.map(HashedPassword::parse).transpose()?,
                first_name: record.first_name.map(FirstName::parse).transpose()?,
                last_name: record.last_name.map(LastName::parse).transpose()?,
                role: record.role,
                gender: record.gender,
                is_verified: record.is_verified,
                is_banned: record.is_banned,
                google_id: record.google_id.map(GoogleID::parse).transpose()?,
                facebook_id: record.facebook_id.map(FacebookID::parse).transpose()?,
            };

            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    #[instrument(skip_all, name = "authrepository - create user")]
    pub async fn create_user(&self, user: &NewUser) -> Result<Uuid> {
        let rec = query!(
            r#"
                INSERT INTO users (email, password, first_name, last_name, gender)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id;
            "#,
            user.email.as_ref(),
            user.hashed_password.as_ref(),
            user.first_name.as_ref().map(|f| f.as_ref()),
            user.last_name.as_ref().map(|l| l.as_ref()),
            user.gender.clone() as Option<UserGender>
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec.id)
    }

    #[instrument(skip_all, name = "authrepository - update user")]
    pub async fn update_user(&self, id: &UserID, user: UpdateUser) -> Result<()> {
        query!(
            r#"
              UPDATE users
              SET first_name = COALESCE($1, first_name),
                  last_name = COALESCE($2, last_name),
                  password = COALESCE($3, password),
                  google_id = COALESCE($4, google_id),
                  facebook_id = COALESCE($5, facebook_id),
                  gender = COALESCE($6, gender),
                  is_verified = COALESCE($7, is_verified),
                  updated_at = NOW()
              WHERE id = $8
            "#,
            user.first_name.map(|s| s.to_string()),
            user.last_name.map(|s| s.to_string()),
            user.password.map(|s| s.to_string()),
            user.google_id.map(|s| s.to_string()),
            user.facebook_id.map(|s| s.to_string()),
            user.gender as Option<UserGender>,
            user.is_verified,
            id.as_ref()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
