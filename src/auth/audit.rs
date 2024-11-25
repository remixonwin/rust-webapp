use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use actix_web::HttpRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub event_type: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct AuditLogger {
    pool: PgPool,
}

impl AuditLogger {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn log_event(
        &self,
        user_id: Option<Uuid>,
        event_type: &str,
        req: &HttpRequest,
        details: Option<serde_json::Value>,
    ) -> Result<(), sqlx::Error> {
        let ip_address = req
            .connection_info()
            .realip_remote_addr()
            .map(|s| s.to_string());

        let user_agent = req
            .headers()
            .get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        sqlx::query!(
            r#"
            INSERT INTO auth_logs (
                user_id, event_type, ip_address, user_agent, details
            ) VALUES ($1, $2, $3, $4, $5)
            "#,
            user_id,
            event_type,
            ip_address,
            user_agent,
            details
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_logs(
        &self,
        user_id: Uuid,
        limit: i64
    ) -> Result<Vec<AuthLog>, sqlx::Error> {
        sqlx::query_as!(
            AuthLog,
            r#"
            SELECT id, user_id, event_type, ip_address, user_agent, created_at, details
            FROM auth_logs
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            user_id,
            limit
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_suspicious_activities(
        &self,
        hours: i64,
        min_attempts: i64
    ) -> Result<Vec<AuthLog>, sqlx::Error> {
        sqlx::query_as!(
            AuthLog,
            r#"
            SELECT id, user_id, event_type, ip_address, user_agent, created_at, details
            FROM auth_logs
            WHERE event_type = 'login_failed'
            AND created_at > NOW() - INTERVAL '1 hour' * $1
            GROUP BY id, user_id, event_type, ip_address, user_agent, created_at, details
            HAVING COUNT(*) >= $2
            ORDER BY created_at DESC
            "#,
            hours,
            min_attempts
        )
        .fetch_all(&self.pool)
        .await
    }
}
