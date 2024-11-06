use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{model::{player::Player, search::Sport, session::{Session, SessionData}, Count, Username}, schema::session::{DeleteDeleteSession, GetSession, GetSessionPlayers, PatchEditSession, PostCreateSession, PostSessionRsvp}, AppState};

pub async fn get_session(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetSession>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let session = sqlx::query_as!(
        SessionData,
        "SELECT ses.id, 
        ses.location_name, 
        ses.session_name, 
        ses.start_time,
        p.username, 
        s.name as sport, 
        s.icon_source as sport_icon_source, 
        ses.lat, ses.lon, 
        ses.end_time, 
        p.profile_picture as username_icon, 
        s.icon as sport_icon, 
        ses.max_players, 
        (SELECT COUNT(player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') as count_rsvps,
        earth_distance(ll_to_earth(ses.lat, ses.lon),ll_to_earth($2, $3)) as dis
         FROM session ses
         INNER JOIN player p ON ses.host_id = p.id
         INNER JOIN sport s ON ses.sport_id = s.id
         WHERE ses.id = $1",
        id,
        body.lat,
        body.lng
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "data": json!({"session": session})})),
    ))
}

pub async fn create_session(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<PostCreateSession>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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
        session (session_name, sport_id, host_id, lat, lon, start_time, public, max_players, location_name, end_time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        body.session_name,
        sport.id,
        player.id,
        body.lat,
        body.lng,
        body.start_time,
        body.public,
        body.max_players,
        body.location_name,
        body.end_time
    )
    .execute(&data.db)
    .await
    .unwrap();

    let player_sport_response =
        json!({"status": "success", "response": "session created successfully"});
    return Ok((StatusCode::CREATED, Json(player_sport_response)));
}


pub async fn edit_session(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<PatchEditSession>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    if body.lat.is_some() && body.lng.is_some() {
        let lat = body.lat.unwrap();
        let lng = body.lng.unwrap();
        let _ = sqlx::query_as!(
            Session,
            "UPDATE session SET lat = $1, lon = $2 WHERE id = $3 AND host_id = $4",
            lat,
            lng,
            body.session_id as Uuid,
            player.id
        )
        .execute(&data.db)
        .await
        .unwrap();
    }

    if body.session_name.is_some() {
        let session_name = body.session_name.unwrap();
        let _ = sqlx::query_as!(
            Session,
            "UPDATE session SET session_name = $1 WHERE id = $2 AND host_id = $3",
            session_name,
            body.session_id as Uuid,
            player.id
        )
        .execute(&data.db)
        .await
        .unwrap();
    }

    if body.end_time.is_some() {
        let end_time = body.end_time.unwrap();
        let _ = sqlx::query_as!(
            Session,
            "UPDATE session SET end_time = $1 WHERE id = $2 AND host_id = $3",
            end_time,
            body.session_id as Uuid,
            player.id
        )
        .execute(&data.db)
        .await
        .unwrap();
    }

    if body.start_time.is_some() {
        let start_time = body.start_time.unwrap();
        let _ = sqlx::query_as!(
            Session,
            "UPDATE session SET start_time = $1 WHERE id = $2 AND host_id = $3",
            start_time,
            body.session_id as Uuid,
            player.id
        )
        .execute(&data.db)
        .await
        .unwrap();
    }
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "response": "session updated successfully"})),
    ))
}


pub async fn delete_session(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<DeleteDeleteSession>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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
        "DELETE FROM session WHERE id = $1 AND host_id = $2",
        body.session_id,
        player.id
    )
    .execute(&data.db)
    .await
    .unwrap();
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","message": "session deleted successfully"})),
    ))
}

pub async fn rsvp_session(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<PostSessionRsvp>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.player_username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let session = sqlx::query_as!(
        Session,
        "SELECT * FROM session WHERE id = $1",
        body.session_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let accepted_sessions = sqlx::query_as!(
        Count,
        "SELECT COUNT(sr.player_id) as count FROM session_rsvp sr
        INNER JOIN session s ON s.id = sr.session_id
        WHERE sr.player_rsvp = 'Yes' AND s.id = $1",
        session.id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let rsvp_player = accepted_sessions.count.unwrap_or(0) + 1;

    if session.max_players > rsvp_player as i32 && session.host_id != player.id {
        let _ = sqlx::query_as!(
            SessionRsvp,
            "INSERT INTO
            session_rsvp (session_id, player_id, player_rsvp, host_rsvp)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (session_id, player_id)
            DO UPDATE
            SET player_rsvp = $3
            ",
            body.session_id,
            player.id,
            body.player_rsvp,
            "Yes"
        )
        .execute(&data.db)
        .await
        .unwrap();
        Ok((
            StatusCode::OK,
            Json(json!({"status": "success","message": "rsvp recorded successfully"})),
        ))
    } else {
        Ok((
            StatusCode::NOT_ACCEPTABLE,
            Json(json!({"status": "success","message": "rsvp max limit reached for session"})),
        ))
    }
}

pub async fn session_players(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetSessionPlayers>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let host = sqlx::query_as!(
        Username,
        "SELECT p.username
        FROM session ses
        INNER JOIN player p ON ses.host_id = p.id 
        WHERE ses.id = $1",
        body.session_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    let mut usernames = sqlx::query_as!(
        Username,
        "SELECT p.username
        FROM session_rsvp sr
        INNER JOIN player p ON p.id = sr.player_id
        WHERE sr.session_id = $1 AND sr.player_rsvp = 'Yes'",
        body.session_id
    )
    .fetch_all(&data.db)
    .await
    .unwrap_or(Vec::new());
    usernames.push(host);

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","data": json!({"players": usernames})})),
    ))
}