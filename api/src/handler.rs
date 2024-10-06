use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::{model::{Player, Sport, PlayerSport, Rating}, schema::{CreatePlayer, DeletePlayer, EditPlayer, CreatePlayerSport}, AppState};

pub async fn health_checker() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

// USER

pub async fn create_player
    (
        State(data): State<Arc<AppState>>,
        axum::extract::Json(body): axum::extract::Json<CreatePlayer>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

        let query_result = sqlx::query_as!(
            Player,
            "INSERT INTO player 
            (username, password, date_of_birth) 
            VALUES ($1, $2, $3)
            RETURNING *",
            body.username.to_string(),
            body.password.to_string(),
            body.date_of_birth)
        .fetch_one(&data.db)
        .await;

        match query_result {
            Ok(player) => {
                let player_response = json!({"status": "success", "data": json!({
                    "player": player
                })});
                Ok((StatusCode::CREATED, Json(player_response)))
            }
            Err(e) => {
                if e.to_string()
                    .contains("duplicate key value violates unique constraint")
                {
                    let error_response = serde_json::json!({
                        "status": "fail",
                        "message": "Player with that username already exists",
                    });
                    return Err((StatusCode::CONFLICT, Json(error_response)));
                }
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"status": "error","message": format!("{:?}", e)})),
                ))
            }
        }
}

pub async fn delete_player
    (
        State(data): State<Arc<AppState>>,
        axum::extract::Json(body): axum::extract::Json<DeletePlayer>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        let rows_affected = sqlx::query_as!(
            Player, 
            "DELETE FROM player WHERE username = $1", 
            body.username.to_string())
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Player with username: {} not found", body.username)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn edit_player (
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<EditPlayer>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        Player,
        "UPDATE player SET password = $1 WHERE username = $2 RETURNING *",
        body.password.to_string(),
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(player) => {
            let player_response = json!({"status": "success", "data": json!({
                "player": player
            })});
            Ok((StatusCode::OK, Json(player_response)))
        },
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ))
        }
    }
}

pub async fn get_players (
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(Player, "SELECT * FROM player")
    .fetch_all(&data.db)
    .await;

    match query_result {
        Ok(players) => {
            let player_response = json!({"status": "success", "data": json!({
                "players": players
            })});
            Ok((StatusCode::OK, Json(player_response)))
        },
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ))
        }
    }
}



pub async fn create_player_sport(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<CreatePlayerSport>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sport = sqlx::query_as!(
        Sport, 
        "SELECT * FROM sport WHERE name = $1",
        body.sport.to_string()
    )
    .fetch_one(&data.db)
    .await;

    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match (sport, player) {
        (Ok(sport), Ok(player)) => {

            let player_sport = sqlx::query_as!(
                PlayerSport,
                "INSERT INTO
                player_sport (player_id, sport_id)
                VALUES ($1, $2)
                RETURNING *",
                player.id,
                sport.id
            ).fetch_one(&data.db)
            .await;

            match player_sport {
                Ok(player_sport) => {
                    let rating = sqlx::query_as!(
                        Rating,
                        "INSERT INTO
                        rating (player_sport_id)
                        VALUES ($1)
                        RETURNING *",
                        player_sport.id,
                    ).fetch_one(&data.db)
                    .await;

                    match rating {
                        Ok(_) => {
                            let player_sport_response = json!({"status": "success", "data": "player_sport and rating added successfully"});
                            return Ok((StatusCode::CREATED, Json(player_sport_response)));
                        },
                        Err(e) => {
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"status": "error","message": format!("{:?}", e)}))
                            ));
                        }
                    }
                },
                Err(e) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"status": "error","message": format!("{:?}", e)}))
                    ));
                }
            }
        },
        (Err(sport_err), _) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"status": "error","message": format!("No sport found {:?}", body.sport), "error message": format!("{:?}", sport_err)})),
            ));
        },
        (_, Err(player_err)) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"status": "error","message": format!("No player found {:?}", body.username), "error_message": format!("{:?}", player_err)})),
            ));
        }
    }
}