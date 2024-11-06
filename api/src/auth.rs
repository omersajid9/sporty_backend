use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::Response,
    http::StatusCode,
    RequestPartsExt,
    middleware::Next,
    http::Request,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use axum::body::Body;

use crate::error::AuthError;

// Our claims struct for JWT payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid, // Subject (usually user_id)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

const JWT_SECRET: &[u8] = b"our-secret-key"; // In production, use proper secret management

impl Claims {
    pub fn new(user_id: Uuid, expires_in_seconds: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Self {
            sub: user_id,
            exp: now + expires_in_seconds as usize,
            iat: now,
        }
    }

    pub fn generate_token(&self) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(JWT_SECRET),
        )
    }

    pub fn decode_token(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}

// Middleware function to handle JWT authentication
pub async fn require_auth(
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = Claims::decode_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut request = request;
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingToken)?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::default(),
        )
        .map_err(AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}