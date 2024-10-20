use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use sqlx::{Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{
    model::{
        Count, Game, GameData, GameUsername, MatchUsernameData, Player, PlayerSport, Rating, RatingData, Score, ScoreData, Session, SessionData, SessionRsvpUsername, Sport
    },
    schema::{
        ConfirmScore, CreatePlayer, CreatePlayerSport, CreateSession, CreateSessionRsvp, DeletePlayer, DeletePlayerSport, DeleteSession, EditPlayer, EditSession, GetMatch, GetSessionUsernames, GetSessions, GetUpcomingSessions, GetUserProfile, ReportScores, Rsvp, SearchPlayers, SignIn, SignUp
    },
    AppState,
};

use skillratings::{
    glicko2::{glicko2, Glicko2Config, Glicko2Rating},
    Outcomes,
};

pub async fn health_checker() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Ok((StatusCode::CREATED, Json(json_response)))
}

// SPORT

pub async fn get_sports_all(
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

// USER
pub async fn sign_in(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<SignIn>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
        // Try to query the player with the provided username and password
    let result = sqlx::query_as!(
        Player,
        "SELECT * 
        FROM player 
        WHERE username = $1 AND password = $2",
        body.username.to_string(),
        body.password.to_string(),
    )
    .fetch_one(&data.db)
    .await;

    // Match on the result to handle success or failure
    match result {
        Ok(player) => {
            // If successful, return a success message
            let player_response = json!({
                "status": "success", 
                "message": "player signed in successfully",
                "data": {
                    "username": player.username,
                }
            });
            Ok((StatusCode::OK, Json(player_response)))
        }
        Err(e) => {
            // Handle case where no matching player is found (e.g., wrong username or password)
            if let sqlx::Error::RowNotFound = e {
                let error_response = json!({
                    "status": "error", 
                    "message": "invalid username or password"
                });
                return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
            }

            // For other errors, return a generic error message
            let error_response = json!({"status": "error", "message": "an error occurred"});
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

pub async fn sign_up(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<SignUp>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("SING UP BODY: {:?}", body);
    // Try to insert the new player into the database
    let result = sqlx::query_as!(
        Player,
        "INSERT INTO player 
            (username, password, date_of_birth, profile_picture) 
            VALUES ($1, $2, $3, $4)
            RETURNING *",
        body.username.to_string(),
        body.password.to_string(),
        body.date_of_birth,
        body.profile_picture
    )
    .fetch_one(&data.db)
    .await;

    // Match on the result to handle success or failure
    match result {
        Ok(_player) => {
            // If successful, return a success message
            let player_response = json!({"status": "success", "data": "player created successfully"});
            Ok((StatusCode::CREATED, Json(player_response)))
        }
        Err(e) => {
            // Handle specific SQL errors, like a duplicate username
            if let sqlx::Error::Database(db_err) = &e {
                // Check if it's a unique violation error
                if db_err.constraint() == Some("player_username_key") {
                    let error_response = json!({"status": "error", "message": "username already taken"});
                    return Err((StatusCode::CONFLICT, Json(error_response)));
                }
            }
            // For other errors, return a generic error message
            let error_response = json!({"status": "error", "message": "an error occurred"});
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}


pub async fn create_player(
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
        body.date_of_birth
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let player_response = json!({"status": "success", "data": "player created successfully"});
    Ok((StatusCode::CREATED, Json(player_response)))
}

pub async fn delete_player(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<DeletePlayer>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query_as!(
        Player,
        "DELETE FROM player WHERE username = $1",
        body.username.to_string()
    )
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

pub async fn edit_player(
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

pub async fn get_players(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<SearchPlayers>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut players: Vec<Player> = Vec::new();
    if body.query == "*" {
        players = sqlx::query_as!(
            Player,
            "SELECT * FROM player
            "
        )
        .fetch_all(&data.db)
        .await
        .unwrap();
    } else {
        let search_query = format!("%{}%", body.query);
        players = sqlx::query_as!(
            Player,
            "SELECT * FROM player
            WHERE username ILIKE $1
            ",
            search_query
        )
        .fetch_all(&data.db)
        .await
        .unwrap();
    }
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

    // let player_sport = sqlx::query_as!(
    //     PlayerSport,
    //     "INSERT INTO
    //     player_sport (player_id, sport_id)
    //     VALUES ($1, $2)
    //     RETURNING *",
    //     player.id,
    //     sport.id
    // )
    // .fetch_one(&data.db)
    // .await
    // .unwrap();

    // let _ = sqlx::query_as!(
    //     Rating,
    //     "INSERT INTO
    //     rating (player_sport_id)
    //     VALUES ($1)
    //     RETURNING *",
    //     player_sport.id,
    // )
    // .fetch_one(&data.db)
    // .await
    // .unwrap();

    let player_sport_response =
        json!({"status": "success", "data": "player_sport and rating added successfully"});
    return Ok((StatusCode::CREATED, Json(player_sport_response)));
}

pub async fn delete_player_sport(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<DeletePlayerSport>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Implement this function using SQLX query to delete player_sport and rating associated with it.
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let sport = sqlx::query_as!(
        Sport,
        "SELECT * FROM sport WHERE name = $1",
        body.sport.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    // let player_sport = sqlx::query_as!(
    //     PlayerSport,
    //     "DELETE FROM player_sport WHERE player_id = $1 AND sport_id = $2 RETURNING *",
    //     player.id,
    //     sport.id
    // )
    // .fetch_one(&data.db)
    // .await
    // .unwrap();

    // let _ = sqlx::query_as!(
    //     Rating,
    //     "DELETE FROM rating WHERE player_sport_id = $1",
    //     player_sport.id
    // )
    // .execute(&data.db)
    // .await
    // .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "response": "Player Sport deleted successfully"})),
    ))
}

pub async fn update_player_rating(
    State(_data): State<Arc<AppState>>,
    // axum::extract::Json(body): axum::extract::Json<CreatePlayerSport>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Implement this function using SQLX query to delete player_sport and rating associated with it.
    Ok((
        StatusCode::OK,
        Json(json!({"status": "ok","message": "hey"})),
    ))
}

// Game

pub async fn get_session(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {


    let session = sqlx::query_as!(
        SessionData,
        "SELECT ses.id, ses.location_name, ses.session_name, p.username, s.name as sport, s.icon_source as sport_icon_source, ses.lat, ses.lon, ses.time, p.profile_picture as username_icon, s.icon as sport_icon, ses.max_players, (SELECT COUNT(player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') as count_rsvps
        ,earth_distance(ll_to_earth(ses.lat, ses.lon),ll_to_earth(40.730610, -73.935242)) as dis
         FROM session ses
         INNER JOIN player p ON ses.host_id = p.id
         INNER JOIN sport s ON ses.sport_id = s.id
         WHERE ses.id = $1",
        id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "data": json!({"game": session})})),
    ))
}

pub async fn get_sessions(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetSessions>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("{:?}", body);

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

    let sessions = query
        .build_query_as::<SessionData>()
        .fetch_all(&data.db)
        .await
        .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "data": json!({"sessions": sessions})})),
    ))
}

pub async fn create_session(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<CreateSession>,
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
        session (session_name, sport_id, host_id, lat, lon, time, public, max_players, location_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        body.name,
        sport.id,
        player.id,
        body.location.lat,
        body.location.lon,
        body.time,
        body.public,
        body.max_players,
        body.location_name
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
    axum::extract::Json(body): axum::extract::Json<EditSession>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
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
        )
        .execute(&data.db)
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
    axum::extract::Json(body): axum::extract::Json<DeleteSession>,
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
        body.game_id,
        player.id
    )
    .execute(&data.db)
    .await
    .unwrap();
    // Implement this function using SQLX query to delete player_sport and rating associated with it.
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","message": "session deleted successfully"})),
    ))
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
        WHERE sr.player_rsvp = 'Yes' AND sr.host_rsvp = 'Yes' AND s.id = $1",
        session.id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let rsvp_player = accepted_sessions.count.unwrap_or(0) + 1;

    let mut host_rsvp = Rsvp::Pending;
    if !session.public {
        host_rsvp = Rsvp::Yes;
    }

    if session.max_players > rsvp_player as i32 {
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

pub async fn get_upcoming_player_sessions(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetUpcomingSessions>,
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
        SessionData,
        "SELECT ses.id, ses.location_name, ses.session_name, p.username, s.name as sport, s.icon_source as sport_icon_source, ses.lat, ses.lon, ses.time, p.profile_picture as username_icon, s.icon as sport_icon, ses.max_players, (SELECT COUNT(player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') as count_rsvps
        ,earth_distance(ll_to_earth(ses.lat, ses.lon),ll_to_earth(40.730610, -73.935242)) as dis
         FROM session ses
         INNER JOIN player p ON ses.host_id = p.id
         INNER JOIN sport s ON ses.sport_id = s.id
         WHERE ses.id IN (SELECT session_id FROM session_rsvp WHERE player_id = $1 AND player_rsvp = 'Yes')
         OR ses.host_id = $1
         ORDER BY ses.time ASC",
        player.id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","data": json!({"sessions": sessions})})),
    ))
}

pub async fn get_session_rsvp_usernames(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetSessionUsernames>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let host = sqlx::query_as!(
        SessionRsvpUsername,
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
        SessionRsvpUsername,
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
        Json(json!({"status": "success","data": json!({"usernames": usernames})})),
    ))
}

pub async fn get_sessions_played(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetUpcomingSessions>,
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
        SessionData,
        "SELECT ses.id, ses.location_name, ses.session_name, p.username, s.name as sport, s.icon_source as sport_icon_source, ses.lat, ses.lon, ses.time, p.profile_picture as username_icon, s.icon as sport_icon, ses.max_players, (SELECT COUNT(player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') as count_rsvps
        ,earth_distance(ll_to_earth(ses.lat, ses.lon),ll_to_earth(40.730610, -73.935242)) as dis
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

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","data": json!({"sessions": sessions})})),
    ))
}


pub async fn report_score(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<ReportScores>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    for report in body.scores {
        let player_1 = sqlx::query_as!(
            Player,
            "SELECT * FROM player WHERE username = $1",
            report.username.to_string()
        )
        .fetch_one(&data.db)
        .await
        .unwrap();

        let player_2 = sqlx::query_as!(
            Player,
            "SELECT * FROM player WHERE username = $1",
            report.opponent_username.to_string()
        )
        .fetch_one(&data.db)
        .await
        .unwrap();

    let game = sqlx::query_as!(
        Game,
        "INSERT INTO
        game (session_id, player_id_1, player_id_2)
        VALUES ($1, $2, $3)
        RETURNING *",
        report.session_id,
        player_1.id,
        player_2.id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    
    for (index, score) in report.score.iter().enumerate() {
            let _ = sqlx::query_as!(
                Score,
                "INSERT INTO
                score (game_id, player_id, score, round)
                VALUES ($1, $2, $3, $4)
                RETURNING *",
                game.id,
                player_1.id,
                score[0],
                index as i32
            )
            .fetch_one(&data.db)
            .await
            .unwrap();

            let _ = sqlx::query_as!(
                Score,
                "INSERT INTO
                score (game_id, player_id, score, round)
                VALUES ($1, $2, $3, $4)
                RETURNING *",
                game.id,
                player_2.id,
                score[1],
                index as i32
            )
            .fetch_one(&data.db)
            .await
            .unwrap();
        }
    }
    Ok((
        StatusCode::OK,
        Json(json!({"status": "success","message": "score reported successfully"})),
    ))
}

pub async fn confirm_score(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<ConfirmScore>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let game = sqlx::query_as!(
        Game,
        "SELECT *
        FROM game WHERE id = $1",
        body.game_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    if game.player_id_2 == player.id {
        let _ = sqlx::query_as!(
            Game,
            "UPDATE game SET status = $1 WHERE id = $2",
            get_rsvp_str(body.confirmation),
            body.game_id
        )
        .execute(&data.db)
        .await
        .unwrap();

        let rating_history_1 = sqlx::query_as!(
            Rating,
            "SELECT * 
            FROM rating r
            WHERE r.player_id = $1
            AND r.sport_id IN (SELECT ses.sport_id FROM game g INNER JOIN session ses ON g.session_id = ses.id WHERE g.id = $2) ",
            game.player_id_1,
            game.id
        ).fetch_optional(&data.db)
        .await
        .unwrap();

        let rating_history_2 = sqlx::query_as!(
            Rating,
            "SELECT * 
            FROM rating r
            WHERE r.player_id = $1
            AND r.sport_id IN (SELECT ses.sport_id FROM game g INNER JOIN session ses ON g.session_id = ses.id WHERE g.id = $2) ",
            game.player_id_2,
            game.id
        ).fetch_optional(&data.db)
        .await
        .unwrap();

        let mut rating_1 = match rating_history_1 {
            Some(rating) => Glicko2Rating {
                rating: rating.rating,
                deviation: rating.std,
                volatility: rating.val,
            },
            None => Glicko2Rating::default()
        };

        let mut rating_2 = match rating_history_2 {
            Some(rating) => Glicko2Rating {
                rating: rating.rating,
                deviation: rating.std,
                volatility: rating.val,
            },
            None => Glicko2Rating::default()
        };

        let (player_id_1, player_id_2) = if player.id == game.player_id_1 {
            (game.player_id_1, game.player_id_2)
        } else {
            (game.player_id_2, game.player_id_1)
        };

        let scores = sqlx::query_as!(
            ScoreData,
            "SELECT s1.score as score_1, s2.score as score_2
            FROM game g
            INNER JOIN score s1 ON s1.game_id = g.id AND s1.player_id = $1
            INNER JOIN score s2 ON s2.game_id = g.id AND s2.player_id = $2 AND s1.round = s2.round
            WHERE g.id = $3",
            player_id_1,
            player_id_2,
            game.id
        )
        .fetch_all(&data.db)
        .await
        .unwrap();

        for score in scores {
            let outcome = if score.score_1 > score.score_2 {
                Outcomes::WIN
            } else if score.score_1 < score.score_2 {
                Outcomes::LOSS
            } else {
                Outcomes::DRAW
            };
            let config = Glicko2Config::new();
            (rating_1, rating_2) = glicko2(&rating_1, &rating_2, &outcome, &config);
        }

        let _ = sqlx::query_as!(
            Rating,
            "INSERT INTO
            rating (player_id, sport_id, rating, std, val)
            VALUES ($1, (SELECT ses.sport_id FROM game g INNER JOIN session ses ON g.session_id = ses.id WHERE g.id = $2), $3, $4, $5)
            RETURNING *",
            player_id_1,
            game.id,
            rating_1.rating,
            rating_1.deviation,
            rating_1.volatility
        )
        .fetch_one(&data.db)
        .await
        .unwrap();

        let _ = sqlx::query_as!(
            Rating,
            "INSERT INTO
            rating (player_id, sport_id, rating, std, val)
            VALUES ($1, (SELECT ses.sport_id FROM game g INNER JOIN session ses ON g.session_id = ses.id WHERE g.id = $2), $3, $4, $5)
            RETURNING *",
            player_id_2,
            game.id,
            rating_2.rating,
            rating_2.deviation,
            rating_2.volatility
        )
        .fetch_one(&data.db)
        .await
        .unwrap();

        Ok((
            StatusCode::OK,
            Json(json!({"status": "success","message": "score confirmed successfully"})),
        ))
    } else {
        Ok((
            StatusCode::NOT_ACCEPTABLE,
            Json(json!({"status": "success","message": "score confirmed unsuccefully"})),
        ))
    }
}

pub async fn get_user_profile(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetUserProfile>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE username = $1",
        body.username.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let ratings = sqlx::query_as!(
        RatingData,
        "SELECT r.std as std, r.val, r.rating, r.updated, s.name as sport, s.icon as sport_icon, s.icon_source as sport_icon_source
        FROM rating r
        INNER JOIN sport s ON r.sport_id = s.id
        WHERE r.player_id = $1",
        player.id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let games = sqlx::query_as!(
        Game,
        "SELECT *
        FROM game g
        WHERE g.player_id_1 = $1 OR g.player_id_2 = $1",
        player.id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let mut games_data: Vec<GameData> = Vec::new();

    for game in games {
        let (player_id_1, player_id_2) = if player.id == game.player_id_1 {
            (game.player_id_1, game.player_id_2)
        } else {
            (game.player_id_2, game.player_id_1)
        };

        let opponent = sqlx::query_as!(
            Player,
            "SELECT * FROM player WHERE id = $1",
            player_id_2
        )
        .fetch_one(&data.db)
        .await
        .unwrap();
        
        let scores = sqlx::query_as!(
            ScoreData,
            "SELECT s1.score as score_1, s2.score as score_2
            FROM game g
            INNER JOIN score s1 ON s1.game_id = g.id AND s1.player_id = $1
            INNER JOIN score s2 ON s2.game_id = g.id AND s2.player_id = $2 AND s1.round = s2.round
            WHERE g.id = $3",
            player_id_1,
            player_id_2,
            game.id
        )
        .fetch_all(&data.db)
        .await
        .unwrap();

        let g_data = GameData {
            id: game.id,
            player_id_1: game.player_id_1,
            player_id_2: game.player_id_2,
            username_1: player.username.clone(),
            username_2: opponent.username.clone(),
            status: game.status,
            scores: scores,
            created_at: game.created_at,
        };
        games_data.push(g_data);
    }

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "data": json!({"profile": player, "games": games_data, "ratings": ratings})})),
    ))
}



pub async fn get_match(
    State(data): State<Arc<AppState>>,
    axum::extract::Path(body): axum::extract::Path<GetMatch>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let game = sqlx::query_as!(
        GameUsername,
        "SELECT g.*, p1.username as username_1, p2.username as username_2
        FROM game g
        INNER JOIN player p1 ON p1.id = g.player_id_1
        INNER JOIN player p2 ON p2.id = g.player_id_2
        WHERE g.id = $1",
        body.id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let scores = sqlx::query_as!(
        ScoreData,
        "SELECT s1.score as score_1, s2.score as score_2
        FROM game g
        INNER JOIN score s1 ON s1.game_id = g.id AND s1.player_id = g.player_id_1
        INNER JOIN score s2 ON s2.game_id = g.id AND s2.player_id = g.player_id_2 AND s1.round = s2.round
        WHERE g.id = $1",
        body.id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let g_data = GameData {
        id: game.id,
        player_id_1: game.player_id_1,
        player_id_2: game.player_id_2,
        username_1: game.username_1,
        username_2: game.username_2,
        status: game.status,
        scores: scores,
        created_at: game.created_at,
    };

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "data": json!({"game": g_data})})),
    ))

}