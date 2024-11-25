use actix_web::{
    web::{self, Data, Json, ServiceConfig},
    HttpResponse,
    HttpRequest,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::app_state::AppState;
use crate::auth::jwt::create_jwt;
use crate::error::AppError;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(handle_register))
            .route("/login", web::post().to(handle_login))
    );
}

async fn handle_register(
    data: Data<AppState>,
    req: HttpRequest,
    json_req: Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    // Validate request
    if let Err(_) = json_req.validate() {
        return Err(AppError::ValidationError);
    }

    // Check rate limit
    if let Err(_) = data.rate_limiter.check_rate_limit(&req, &json_req.email) {
        return Err(AppError::RateLimit);
    }

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(json_req.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return Err(AppError::Internal),
    };

    // Create user
    match data.user_repository.create_user(&json_req.email, &password_hash).await {
        Ok(user) => Ok(HttpResponse::Created().json(json!({
            "message": "User created successfully",
            "user_id": user.id.to_string()
        }))),
        Err(e) => {
            if e.to_string().contains("duplicate key") {
                Err(AppError::UserExists)
            } else {
                Err(AppError::Internal)
            }
        }
    }
}

async fn handle_login(
    data: Data<AppState>,
    req: HttpRequest,
    credentials: Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    // Check rate limit
    if let Err(_) = data.rate_limiter.check_rate_limit(&req, &credentials.email) {
        return Err(AppError::RateLimit);
    }

    // Find user
    let user = match data.user_repository.get_user_by_email(&credentials.email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::Unauthorized),
        Err(_) => return Err(AppError::Internal),
    };

    // Verify password
    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => hash,
        Err(_) => return Err(AppError::Internal),
    };

    if Argon2::default()
        .verify_password(credentials.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AppError::Unauthorized);
    }

    // Create JWT
    match create_jwt(user.id, &user.email) {
        Ok(token) => Ok(HttpResponse::Ok().json(AuthResponse { token })),
        Err(_) => Err(AppError::JwtError(()))
    }
}
