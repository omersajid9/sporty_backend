use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::{Postgres, QueryBuilder};

use crate::{model::{player::Player, search::{ExploreSessionsData, GoingSessionData, ReportableSessionData, Sport}}, schema::search::{GetExploreSessions, GetGoingSessions, GetPlayers, GetReportableSessions}, AppState};


pub async fn players(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetPlayers>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let players: Vec<Player> = if body.query == "*" {
        sqlx::query_as!(
            Player,
            "SELECT * FROM player"
        )
        .fetch_all(&data.db)
        .await
        .unwrap()
    } else {
        let search_query = format!("%{}%", body.query);
        sqlx::query_as!(
            Player,
            "SELECT * FROM player 
            WHERE username ILIKE $1",
            search_query
        )
        .fetch_all(&data.db)
        .await
        .unwrap()
    };
    let player_response = json!({"status": "success", "data": json!({"players": players})});
    Ok((StatusCode::OK, Json(player_response)))
}

pub async fn sports(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sports = sqlx::query_as!(Sport, "SELECT * FROM sport")
        .fetch_all(&data.db)
        .await
        .unwrap();
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "data": json!({"sports": sports})})),
    ))
}


pub async fn explore_sessions(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetExploreSessions>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT ses.id, ses.location_name, ses.session_name, s.icon_source as sport_icon_source, p.username, s.name as sport, ses.lat, ses.lon, ses.time, p.profile_picture as username_icon, s.icon as sport_icon, ses.max_players, (SELECT COUNT(player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') as count_rsvps
         ");
        query.push(", earth_distance(ll_to_earth(ses.lat, ses.lon), ll_to_earth(").push_bind(body.lat).push(", ").push_bind(body.lng).push(")) as dis ");
    
    query.push("FROM session ses
         INNER JOIN player p ON ses.host_id = p.id
         INNER JOIN sport s ON ses.sport_id = s.id");

    query
        .push(" WHERE ses.id NOT IN (SELECT session_id FROM session_rsvp WHERE player_id = ")
        // .push(distance)
        .push_bind(player.id).push(")").push(" AND ses.host_id <> ").push_bind(player.id).push(" AND (SELECT COUNT(player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') + 1 < ses.max_players")
        
        .push(" AND earth_distance(
        ll_to_earth(ses.lat, ses.lon), 
        ll_to_earth(").push_bind(body.lat).push(", ").push_bind(body.lng).push(")) <= 500 * 1609.34 ");

    if body.sport.clone() != "all" {
        query.push(" AND s.key = ").push_bind(body.sport);
    };

    if let Some(date) = body.date {
        query.push(" AND DATE(ses.time) = ").push_bind(date);
    };

    query.push(" ORDER BY dis ASC, DATE(ses.time) ASC");

    let sessions = query
        .build_query_as::<ExploreSessionsData>()
        .fetch_all(&data.db)
        .await
        .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "data": json!({"sessions": sessions})})),
    ))
}


pub async fn going_sessions(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetGoingSessions>, // add lat lon to get accuracte dis
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let sessions = sqlx::query_as!(
        GoingSessionData,
        "SELECT ses.id, 
        ses.location_name, 
        ses.session_name, 
        p.username, 
        s.name as sport, 
        s.icon_source as sport_icon_source, 
        ses.lat, ses.lon, 
        ses.time, 
        p.profile_picture as username_icon, 
        s.icon as sport_icon, 
        ses.max_players, 
        (SELECT COUNT(player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') as count_rsvps,
        earth_distance(ll_to_earth(ses.lat, ses.lon),ll_to_earth($2, $3)) as dis
         FROM session ses
         INNER JOIN player p ON ses.host_id = p.id
         INNER JOIN sport s ON ses.sport_id = s.id
         WHERE ses.id IN (SELECT session_id FROM session_rsvp WHERE player_id = $1 AND player_rsvp = 'Yes')
         OR ses.host_id = $1
         ORDER BY ses.time ASC",
        player.id,
        body.lat,
        body.lng
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","data": json!({"sessions": sessions})})),
    ))
}


pub async fn reportable_sessions(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetReportableSessions>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    
    let sessions = sqlx::query_as!(
        ReportableSessionData,
        "SELECT ses.id, 
        ses.session_name, 
        s.name as sport, 
        s.icon_source as sport_icon_source, 
        s.icon as sport_icon
         FROM session ses
         INNER JOIN player p ON ses.host_id = p.id
         INNER JOIN sport s ON ses.sport_id = s.id
         WHERE (ses.id IN (SELECT session_id FROM session_rsvp WHERE player_id = $1 AND player_rsvp = 'Yes')
         OR ses.host_id = $1)
         AND (SELECT COUNT(distinct player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') > 0
         ORDER BY ses.time ASC",
         player.id
        )
        .fetch_all(&data.db)
        .await
        .unwrap();
// AND DATE(ses.time) > DATE(NOW() - INTERVAL '3 days')
// AND ses.time <= NOW()

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","data": json!({"sessions": sessions})})),
    ))
}
