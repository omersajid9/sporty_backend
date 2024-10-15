use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{model::{Count, Player, PlayerSport, Rating, Session, SessionData, SessionRsvp, Sport}, schema::{CreatePlayer, CreatePlayerSport, CreateSession, CreateSessionRsvp, DeletePlayer, DeletePlayerSport, DeleteSession, EditPlayer, EditSession, GetSessionRsvps, GetSessions, Rsvp}, AppState};

pub async fn health_checker() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

// SPORT 

pub async fn get_sports_all
    (
        State(data): State<Arc<AppState>>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

        let sports = sqlx::query_as!(
            Sport,
            "SELECT * FROM sport"
        ).fetch_all(&data.db)
        .await
        .unwrap();
        Ok((StatusCode::OK, Json(json!({"status": "success", "data": json!({"sports": sports})}))))
    }

// USER

pub async fn create_player
    (
        State(data): State<Arc<AppState>>,
        axum::extract::Json(body): axum::extract::Json<CreatePlayer>,
    ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

        let _ = sqlx::query_as!(
            Player,
            "INSERT INTO player 
            (username, password, date_of_birth) 
            VALUES ($1, $2, $3)
            RETURNING *",
            body.username.to_string(),
            body.password.to_string(),
            body.date_of_birth)
        .fetch_one(&data.db)
        .await
        .unwrap();

        let player_response = json!({"status": "success", "data": "player created successfully"});
        Ok((StatusCode::CREATED, Json(player_response)))
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
    let _ = sqlx::query_as!(
        Player,
        "UPDATE player SET password = $1 WHERE username = $2",
        body.password.to_string(),
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let player_response = json!({"status": "success", "data": "password updated successfully"});
    Ok((StatusCode::OK, Json(player_response)))
}

pub async fn get_players (
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let players = sqlx::query_as!(Player, "SELECT * FROM player")
    .fetch_all(&data.db)
    .await
    .unwrap();

    let player_response = json!({"status": "success", "data": json!({"players": players})});
    Ok((StatusCode::OK, Json(player_response)))
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
    .await
    .unwrap();

    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let player_sport = sqlx::query_as!(
        PlayerSport,
        "INSERT INTO
        player_sport (player_id, sport_id)
        VALUES ($1, $2)
        RETURNING *",
        player.id,
        sport.id
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let _ = sqlx::query_as!(
        Rating,
        "INSERT INTO
        rating (player_sport_id)
        VALUES ($1)
        RETURNING *",
        player_sport.id,
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let player_sport_response = json!({"status": "success", "data": "player_sport and rating added successfully"});
    return Ok((StatusCode::CREATED, Json(player_sport_response)));
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

pub async fn get_session(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let session = sqlx::query_as!(
        SessionData, 
        "SELECT ses.id, p.username, s.name as sport, ses.lat, ses.lon, ses.time 
         FROM session ses
         INNER JOIN player p ON ses.host_id = p.id
         INNER JOIN sport s ON ses.sport_id = s.id
         WHERE ses.id = $1",
         id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    Ok((StatusCode::OK, Json(json!({"status": "success", "data": json!({"game": session})}))))
}

pub async fn get_sessions(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetSessions>
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT ses.id, p.username, s.name as sport, ses.lat, ses.lon, ses.time
         FROM session ses
         INNER JOIN player p ON ses.host_id = p.id
         INNER JOIN sport s ON ses.sport_id = s.id
         ");
        
    query.push(" WHERE ses.id NOT IN (SELECT session_id FROM session_rsvp WHERE player_id = ").push_bind(player.id);
    query.push(")");
    query.push(" AND ses.host_id <> ").push_bind(player.id);

    if body.sport.clone() != "all" {
        query.push(" AND s.key = ").push_bind(body.sport);
    };

    if let Some(date) = body.date {
        query.push(" AND DATE(ses.time) = ").push_bind(date);
    };

    let games = query.build_query_as::<SessionData>().fetch_all(&data.db).await.unwrap();
    Ok((StatusCode::OK, Json(json!({"status": "success", "data": json!({"games": games})}))))
}

pub async fn create_session(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<CreateSession>,
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let sport = sqlx::query_as!(
        Sport, 
        "SELECT * FROM sport WHERE key = $1",
        body.sport.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let _ = sqlx::query_as!(
        Session,
        "INSERT INTO
        session (sport_id, host_id, lat, lon, time, public)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *",
        sport.id,
        player.id,
        body.location.lat, body.location.lon,
        body.time,
        body.public
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let player_sport_response = json!({"status": "success", "response": "session created successfully"});
    return Ok((StatusCode::CREATED, Json(player_sport_response)));
}

pub async fn edit_session(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<EditSession>,
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
            Session,
            "UPDATE session SET lat = $1, lon = $2 WHERE id = $3 AND host_id = $4",
            location.lat,
            location.lon,
            body.game_id as Uuid,
            player.id
        ).execute(&data.db)
        .await
        .unwrap();
    }

    if body.time.is_some() {
        let time = body.time.unwrap();
        let _ = sqlx::query_as!(
            Session,
            "UPDATE session SET time = $1 WHERE id = $2 AND host_id = $3",
            time,
            body.game_id as Uuid,
            player.id
        ).execute(&data.db)
        .await
        .unwrap();
    }
    Ok((StatusCode::OK, Json(json!({"status": "success", "response": "session updated successfully"}))))
}

pub async fn delete_session(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<DeleteSession>,
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let _ = sqlx::query_as!(
        Session,
        "DELETE FROM session WHERE id = $1 AND host_id = $2",
        body.game_id,
        player.id
    ).execute(&data.db)
    .await
    .unwrap();
    // Implement this function using SQLX query to delete player_sport and rating associated with it.
    Ok((StatusCode::OK, Json(json!({"status": "success","message": "session deleted successfully"}))))
}


// player joins game

fn get_rsvp_str(rsvp: Rsvp) -> &'static str {
    match rsvp {
        Rsvp::Pending => "Pending",
        Rsvp::Yes => "Yes",
        Rsvp::No => "No",
    }
}

pub async fn session_rsvp(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<CreateSessionRsvp>,
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.player_username.to_string()
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let session = sqlx::query_as!(
        Session,
        "SELECT * FROM session WHERE id = $1",
        body.session_id
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let accepted_sessions = sqlx::query_as!(
        Count,
        "SELECT COUNT(*) as count FROM session_rsvp sr
        INNER JOIN session s ON s.id = sr.session_id
        WHERE sr.player_rsvp = 'Yes' AND sr.host_rsvp = 'Yes'"
    ).fetch_one(&data.db)
    .await
    .unwrap();

    let rsvp_player = accepted_sessions.count.unwrap_or(0) + 1;

    let mut host_rsvp = Rsvp::Pending;
    if !session.public {
        host_rsvp = Rsvp::Yes;
    }

    if session.max_players > rsvp_player {
        let _ = sqlx::query_as!(
            SessionRsvp,
            "INSERT INTO
            session_rsvp (session_id, player_id, player_rsvp, host_rsvp)
            VALUES ($1, $2, $3, $4)
            ",
            body.session_id,
            player.id,
            get_rsvp_str(body.player_rsvp),
            get_rsvp_str(host_rsvp)
        ).execute(&data.db)
        .await
        .unwrap();
        Ok((StatusCode::OK, Json(json!({"status": "success","message": "rsvp recorded successfully"}))))
    } else {
        Ok((StatusCode::NOT_ACCEPTABLE, Json(json!({"status": "success","message": "rsvp max limit reached for session"}))))
    }
}

pub async fn _get_session_rsvps(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<GetSessionRsvps>,
)-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let rsvps = sqlx::query_as!(
        SessionRsvp,
        "SELECT *
        FROM session_rsvp 
        WHERE session_id = $1 AND player_rsvp = $2",
        body.session_id,
        get_rsvp_str(body.rsvp)
    ).fetch_all(&data.db)
    .await
    .unwrap();

    Ok((StatusCode::OK, Json(json!({"status": "success","data": json!({"rsvps": rsvps})}))))
}

// pub async fn join_game(
//     State(data): State<Arc<AppState>>,
//     axum::extract::Json(body): axum::extract::Json<requestSchema>,
// )-> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     // Implement this function using SQLX query to delete player_sport and rating associated with it.
//     Ok((StatusCode::OK, Json(json!({"status": "ok","message": "hey"}))))
// }



// player reports score



