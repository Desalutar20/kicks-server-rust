use sqlx::{PgPool, query};
use tracing::instrument;

use crate::{
    Error, Result,
    features::{
        admin::users::service::get_all_users::GetAllUsersInput,
        auth::{
            EmailAddress, FacebookID, FirstName, GoogleID, HashedPassword, LastName, User,
            UserGender, UserID, UserRole,
        },
    },
};

pub struct AdminUsersRepository {
    pool: PgPool,
}

impl AdminUsersRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip_all, name = "admin_users_repository - get all users")]
    pub async fn get_all_users(&self, filters: GetAllUsersInput) -> Result<(Vec<User>, usize)> {
        let records = query!(
            r#"
                SELECT
                    count(*) OVER() AS total_count,
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
                WHERE
                    ($1::bool IS NULL OR is_banned = $1) AND
                    ($2::bool IS NULL OR is_verified = $2) AND
                    ($3::text IS NULL OR email ILIKE '%' || $3 || '%'
                                     OR first_name ILIKE '%' || $3 || '%'
                                     OR last_name ILIKE '%' || $3 || '%') AND
                    ($4::user_gender IS NULL OR gender = $4) AND
                    ($5::timestamptz IS NULL or created_at >= $5) AND
                    ($6::timestamptz IS NULL or created_at <= $6)
                ORDER BY id
                LIMIT $7
                OFFSET $8;
                "#,
            filters.is_banned,
            filters.is_verified,
            filters.search.as_ref().map(|v| v.as_ref()),
            filters.gender.as_ref() as Option<&UserGender>,
            filters.start_date,
            filters.end_date,
            filters.pagination.limit() as i32,
            filters.pagination.offset() as i32
        )
        .fetch_all(&self.pool)
        .await?;

        let total_count: usize = records
            .first()
            .and_then(|r| r.total_count)
            .unwrap_or(0)
            .try_into()
            .map_err(|_| {
                Error::Internal("Failed to convert total_count from i64 to usize.".into())
            })?;

        let users = records
            .into_iter()
            .map(|record| {
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

                Ok(user)
            })
            .collect::<Result<Vec<User>>>()?;

        Ok((users, total_count))
    }

    #[instrument(skip_all, name = "admin_users_repository - toggle user is banned")]
    pub async fn toggle_user_is_banned(&self, id: UserID) -> Result<()> {
        let _ = query!(
            r#"
                UPDATE users
                SET is_banned = not is_banned,
                    updated_at = NOW()
                WHERE id = $1
            "#,
            id.as_ref()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[instrument(skip_all, name = "admin_users_repository - remove user")]
    pub async fn remove_user(&self, id: UserID) -> Result<()> {
        let _ = query!(
            r#"
                DELETE FROM users
                WHERE id = $1
            "#,
            id.as_ref()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
