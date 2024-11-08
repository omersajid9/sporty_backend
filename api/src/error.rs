use axum::{http::StatusCode, response::{IntoResponse, Response}};

// Define possible authentication errors
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("invalid token")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    #[error("missing authorization header")]
    MissingToken,
    #[error("user not found")]
    UserNotFound,
}

// Implement response conversion for AuthError
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::InvalidToken(_) => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token"),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "User not found"),
        };
        
        (status, message).into_response()
    }
}
