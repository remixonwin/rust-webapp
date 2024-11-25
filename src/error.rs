use actix_web::{error::ResponseError, HttpResponse};
use std::fmt;
use sqlx::Error as SqlxError;
use serde_json::json;
use jsonwebtoken::errors::Error as JwtError;

#[derive(Debug)]
pub enum AppError {
    Database(()),
    NotFound,
    Unauthorized,
    RateLimit,
    Internal,
    ValidationError,
    UserExists,
    JwtError(()),
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(_) => write!(f, "Database error occurred"),
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::RateLimit => write!(f, "Rate limit exceeded"),
            AppError::Internal => write!(f, "Internal server error"),
            AppError::ValidationError => write!(f, "Validation error"),
            AppError::UserExists => write!(f, "User already exists"),
            AppError::JwtError(_) => write!(f, "JWT error"),
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::RowNotFound => AppError::NotFound,
            _ => AppError::Database(()),
        }
    }
}

impl From<JwtError> for AppError {
    fn from(_: JwtError) -> Self {
        AppError::JwtError(())
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(_) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Database error occurred"
                }))
            }
            AppError::NotFound => {
                HttpResponse::NotFound().json(json!({
                    "error": "Resource not found"
                }))
            }
            AppError::Unauthorized => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "Unauthorized"
                }))
            }
            AppError::RateLimit => {
                HttpResponse::TooManyRequests().json(json!({
                    "error": "Rate limit exceeded"
                }))
            }
            AppError::Internal => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Internal server error"
                }))
            }
            AppError::ValidationError => {
                HttpResponse::BadRequest().json(json!({
                    "error": "Validation error"
                }))
            }
            AppError::UserExists => {
                HttpResponse::BadRequest().json(json!({
                    "error": "User already exists"
                }))
            }
            AppError::JwtError(_) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "JWT error"
                }))
            }
        }
    }
}
