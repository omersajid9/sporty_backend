use std::sync::Arc;

use axum::{
    extract::State, http::StatusCode, response::IntoResponse, Json
};

use crate::{schema::follow::{DeleteFollow, PostFollow}, AppState};

pub async fn add_follow(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<PostFollow>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let player = sqlx::query!(
        "SELECT * FROM player WHERE username = $1",
        body.username
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let follower = sqlx::query!(
        "SELECT * FROM player WHERE username = $1",
        body.follower_username
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let _ = sqlx::query_as!(
        Follow,
        "INSERT INTO follow (user_id, follower_id) VALUES ($1, $2)",
        player.id,
        follower.id
    ).execute(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "message": "Followed successfully"
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}


pub async fn delete_follow(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<DeleteFollow>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let player = sqlx::query!(
        "SELECT * FROM player WHERE username = $1",
        body.username
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let follower = sqlx::query!(
        "SELECT * FROM player WHERE username = $1",
        body.follower_username
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let _ = sqlx::query_as!(
        Follow,
        "DELETE FROM follow WHERE user_id = $1 AND follower_id = $2",
        player.id,
        follower.id
    ).execute(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "message": "Followed successfully"
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}
