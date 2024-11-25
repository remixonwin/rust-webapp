use sqlx::{PgPool};
use crate::models::user::User;
use sqlx::Error as SqlxError;
use chrono::Utc;
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn user_exists(&self, email: &str) -> Result<bool, SqlxError> {
        let result = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
            email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.exists.unwrap_or(false))
    }

    pub async fn create_user(&self, email: &str, password_hash: &str) -> Result<User, SqlxError> {
        let now = Utc::now();
        
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, password_hash, created_at, updated_at
            "#,
            Uuid::new_v4(),
            email,
            password_hash,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, SqlxError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, created_at, updated_at 
            FROM users 
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
    }
}
