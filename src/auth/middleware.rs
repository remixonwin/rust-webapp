use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use serde_json::json;

use super::jwt::verify_jwt;

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract the token from the Authorization header
        let auth_header = req.headers().get("Authorization");
        
        let auth_token = match auth_header {
            Some(header) => {
                let header_str = header.to_str().unwrap_or("");
                if header_str.starts_with("Bearer ") {
                    header_str[7..].to_string()
                } else {
                    String::new()
                }
            }
            None => String::new(),
        };

        // Verify the token
        if auth_token.is_empty() {
            let error = actix_web::error::ErrorUnauthorized(json!({
                "message": "No authorization token provided"
            }));
            return Box::pin(async move { Err(error) });
        }

        match verify_jwt(&auth_token) {
            Ok(claims) => {
                // Add the user email to request extensions
                req.extensions_mut().insert(claims.sub);
                let fut = self.service.call(req);
                Box::pin(async move { fut.await })
            }
            Err(_) => {
                let error = actix_web::error::ErrorUnauthorized(json!({
                    "message": "Invalid token"
                }));
                Box::pin(async move { Err(error) })
            }
        }
    }
}
