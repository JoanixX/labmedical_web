#![allow(dead_code)]
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::error::{ApiError, ApiResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Sujeto (email del admin)
    pub exp: i64,    // Tiempo de expiración
    pub iat: i64,    // Emitido en
}

pub fn hash_password(password: &str) -> ApiResult<String> {
    hash(password, DEFAULT_COST)
        .map_err(|e| ApiError::Internal(format!("Failed to hash password: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> ApiResult<bool> {
    verify(password, hash)
        .map_err(|e| ApiError::Internal(format!("Failed to verify password: {}", e)))
}

pub fn generate_jwt(email: &str, secret: &str) -> ApiResult<String> {
    let now = Utc::now();
    let expiration = now + Duration::hours(24);
    
    let claims = Claims {
        sub: email.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ApiError::Internal(format!("Failed to generate JWT: {}", e)))
}

pub fn verify_jwt(token: &str, secret: &str) -> ApiResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| ApiError::Auth(format!("Invalid token: {}", e)))
}