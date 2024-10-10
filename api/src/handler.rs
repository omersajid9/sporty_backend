use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use uuid::Uuid;

use crate::{model::{Game, Player, PlayerSport, Rating, Sport}, schema::{CreateGame, CreatePlayer, CreatePlayerSport, DeleteGame, DeletePlayer, DeletePlayerSport, EditGame, EditPlayer}, AppState};

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


pub async fn delete_player_sport(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<DeletePlayerSport>,
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Implement this function using SQLX query to delete player_sport and rating associated with it.
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let sport = sqlx::query_as!(
        Sport,
        "SELECT * FROM sport WHERE name = $1",
        body.sport.to_string()
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let player_sport = sqlx::query_as!(
        PlayerSport,
        "DELETE FROM player_sport WHERE player_id = $1 AND sport_id = $2 RETURNING *",
        player.id,
        sport.id
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let _  = sqlx::query_as!(
        Rating,
        "DELETE FROM rating WHERE player_sport_id = $1",
        player_sport.id
    ).execute(&data.db)
    .await
    .unwrap();

    Ok((StatusCode::OK, Json(json!({"status": "success", "response": "Player Sport deleted successfully"}))))
}


pub async fn update_player_rating(
    State(_data): State<Arc<AppState>>,
    // axum::extract::Json(body): axum::extract::Json<CreatePlayerSport>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Implement this function using SQLX query to delete player_sport and rating associated with it.
    Ok((StatusCode::OK, Json(json!({"status": "ok","message": "hey"}))))
}

// Game

pub async fn create_game(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<CreateGame>,
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

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

            let game = sqlx::query_as!(
                Game,
                "INSERT INTO
                game (sport_id, host_id, lat, lon, time)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *",
                sport.id,
                player.id,
                body.location.latitude, body.location.longitude,
                body.time
            ).fetch_one(&data.db)
            .await;

            match game {
                Ok(game) => {
                    let player_sport_response = json!({"status": "success", "response": "game created successfully", "data": json!({"game": game})
                    });
                    return Ok((StatusCode::CREATED, Json(player_sport_response)));
                },
                Err(e) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"status": "error","Error in game creation. message": format!("{:?}", e)}))
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

pub async fn edit_game(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<EditGame>,
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    ).fetch_one(&data.db)
    .await
    .unwrap();

    if body.location.is_some() {
        let location = body.location.unwrap();
        let _ = sqlx::query_as!(
            Game,
            "UPDATE game SET lat = $1, lon = $2 WHERE id = $3 AND host_id = $4",
            location.latitude,
            location.longitude,
            body.game_id as Uuid,
            player.id
        ).execute(&data.db)
        .await
        .unwrap();
    }

    if body.time.is_some() {
        let time = body.time.unwrap();
        let _ = sqlx::query_as!(
            Game,
            "UPDATE game SET time = $1 WHERE id = $2 AND host_id = $3",
            time,
            body.game_id as Uuid,
            player.id
        ).execute(&data.db)
        .await
        .unwrap();
    }

    Ok((StatusCode::OK, Json(json!({"status": "success", "response": "game updated successfully"}))))
}

pub async fn delete_game(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<DeleteGame>,
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let _ = sqlx::query_as!(
        Game,
        "DELETE FROM game WHERE id = $1 AND host_id = $2",
        body.game_id,
        player.id
    ).execute(&data.db)
    .await
    .unwrap();
    // Implement this function using SQLX query to delete player_sport and rating associated with it.
    Ok((StatusCode::OK, Json(json!({"status": "success","message": "game deleted successfully"}))))
}



// player joins game

// pub async fn join_game(
//     State(data): State<Arc<AppState>>,
//     axum::extract::Json(body): axum::extract::Json<requestSchema>,
// )-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     // Implement this function using SQLX query to delete player_sport and rating associated with it.
//     Ok((StatusCode::OK, Json(json!({"status": "ok","message": "hey"}))))
// }



// player reports score



