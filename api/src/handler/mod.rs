
use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub mod player;
pub mod search;
pub mod session;
pub mod game;

pub async fn health_checker() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}
