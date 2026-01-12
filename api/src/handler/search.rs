use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use sqlx::{Postgres, QueryBuilder};

use crate::{
    model::{
        follow::Follow,
        game::{Game, GameData, TeamScore, UserDetails},
        player::Player,
        search::{ExploreSessionData, GoingSessionData, ReportableSessionData, Sport},
        Username,
    },
    schema::search::{GetExploreSessions, GetGoingSessions, GetPlayers, GetReportableSessions},
    AppState,
};

pub async fn players(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetPlayers>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let players: Vec<Player> = if body.query == "*" {
        sqlx::query_as!(Player, "SELECT * FROM player")
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
    let mut query: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT ses.id, ses.location_name, ses.session_name, s.icon_source as sport_icon_source, p.username, p.id as user_id, s.name as sport, ses.lat, ses.lon, ses.start_time, p.profile_picture as username_icon, s.icon as sport_icon, ses.max_players, (SELECT COUNT(player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') as count_rsvps, ses.end_time
         ");
    query
        .push(", earth_distance(ll_to_earth(ses.lat, ses.lon), ll_to_earth(")
        .push_bind(body.lat)
        .push(", ")
        .push_bind(body.lng)
        .push(")) as dis ");

    query.push(
        "FROM session ses
         INNER JOIN player p ON ses.host_id = p.id
         INNER JOIN sport s ON ses.sport_id = s.id
          ",
    );

    query
        .push(
            " WHERE earth_distance(
        ll_to_earth(ses.lat, ses.lon), 
        ll_to_earth(",
        )
        .push_bind(body.lat)
        .push(", ")
        .push_bind(body.lng)
        .push(")) <= 500 * 1609.34 ");

    if let Some(user_id) = body.user_id {
        query
            .push(" AND ses.id NOT IN (SELECT session_id FROM session_rsvp WHERE player_id = ")
            // .push(distance)
            .push_bind(user_id.clone()).push(")").push(" AND ses.host_id <> ").push_bind(user_id).push(" AND (SELECT COUNT(player_id) FROM session_rsvp WHERE session_id = ses.id AND player_rsvp = 'Yes') + 1 < ses.max_players");
    }

    if body.sport.clone() != "all" {
        query.push(" AND s.key = ").push_bind(body.sport);
    };

    if let Some(date) = body.date {
        query.push(" AND DATE(ses.start_time) = ").push_bind(date);
    };

    query.push(" AND ses.end_time > NOW() ");

    query.push(" ORDER BY dis ASC, DATE(ses.end_time) ASC");

    let sessions = query
        .build_query_as::<ExploreSessionData>()
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
    // let player = sqlx::query_as!(
    //     Player,
    //     "SELECT * FROM player WHERE username = $1",
    //     body.username.to_string()
    // )
    // .fetch_one(&data.db)
    // .await
    // .unwrap();

    let sessions = sqlx::query_as!(
        GoingSessionData,
        "SELECT ses.id, 
        ses.location_name, 
        ses.session_name, 
        ses.start_time,
        p.username, 
        p.id as user_id,
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
         WHERE (ses.id IN (SELECT session_id FROM session_rsvp WHERE player_id = $1 AND player_rsvp = 'Yes')
         OR ses.host_id = $1)
         AND ses.end_time > NOW()
         ORDER BY ses.start_time ASC",
        body.user_id,
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

pub async fn past_sessions(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetGoingSessions>, // add lat lon to get accuracte dis
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let sessions = sqlx::query_as!(
        GoingSessionData,
        "SELECT ses.id, 
        ses.location_name, 
        ses.session_name, 
        ses.start_time,
        p.username, 
        p.id as user_id,
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
         WHERE (ses.id IN (SELECT session_id FROM session_rsvp WHERE player_id = $1 AND player_rsvp = 'Yes')
         OR ses.host_id = $1)
         AND ses.end_time < NOW()
         ORDER BY ses.start_time ASC",
         body.user_id,
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

// pub async fn community_sessions(
//     State(data): State<Arc<AppState>>,
//     axum::extract::Query(body): axum::extract::Query<GetGoingSessions>, // add lat lon to get accuracte dis
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let player = sqlx::query_as!(
//         Player,
//         "SELECT * FROM player WHERE username = $1",
//         body.username.to_string()
//     )
//     .fetch_one(&data.db)
//     .await
//     .unwrap();

//     let games = sqlx::query_as!(
//         Game,
//         "SELECT g.*
//         FROM game g
//         JOIN team_member tm1 ON tm1.team_id = g.team_id_1
//         JOIN follow f1 ON f1.user_id = tm1.player_id
//         JOIN team_member tm2 ON tm2.team_id = g.team_id_2
//         JOIN follow f2 ON f2.user_id = tm2.player_id
//         WHERE f1.follower_id = $1 OR f2.follower_id = $1
//         ORDER BY g.created_at DESC",
//         player.id
//     )
//     .fetch_all(&data.db)
//     .await
//     .unwrap();

//     let mut games_data: Vec<GameData> = Vec::new();

//     for game in games {
//         let usernames_team_1 = sqlx::query_as!(
//             UserDetails,
//             "SELECT p.username, p.profile_picture
//             FROM team_member tm
//             INNER JOIN player p ON p.id = tm.player_id
//             WHERE tm.team_id = $1",
//             game.team_id_1
//         )
//         .fetch_all(&data.db)
//         .await
//         .unwrap();

//         let usernames_team_2 = sqlx::query_as!(
//             UserDetails,
//             "SELECT p.username, p.profile_picture
//             FROM team_member tm
//             INNER JOIN player p ON p.id = tm.player_id
//             WHERE tm.team_id = $1",
//             game.team_id_2
//         )
//         .fetch_all(&data.db)
//         .await
//         .unwrap();

//         let reporter = sqlx::query_as!(
//             Player,
//             "SELECT * FROM player WHERE id = $1",
//             game.reporter_id
//         )
//         .fetch_one(&data.db)
//         .await
//         .unwrap();

//         let usernames_1: Vec<String> = usernames_team_1
//             .iter()
//             .map(|u| u.username.clone())
//             .collect();
//         let usernames_2: Vec<String> = usernames_team_2
//             .iter()
//             .map(|u| u.username.clone())
//             .collect();
//         let mut players = Vec::new();
//         players.extend(usernames_1);
//         players.extend(usernames_2);

//         let accepted: Vec<String> = sqlx::query_as!(
//             Username,
//             "SELECT p.username
//             FROM player p
//             INNER JOIN score_validation sv ON sv.player_id = p.id
//             WHERE sv.game_id = $1
//             AND sv.status = 'Yes'",
//             game.id
//         )
//         .fetch_all(&data.db)
//         .await
//         .unwrap()
//         .iter()
//         .map(|u| u.username.clone())
//         .collect();

//         let total: Vec<String> = sqlx::query_as!(
//             Username,
//             "SELECT p.username
//             FROM player p
//             INNER JOIN score_validation sv ON sv.player_id = p.id
//             WHERE sv.game_id = $1",
//             game.id
//         )
//         .fetch_all(&data.db)
//         .await
//         .unwrap()
//         .iter()
//         .map(|u| u.username.clone())
//         .collect();

//         // let total: i64 = confirmations.unwrap().total.unwrap_or(0) + 1;
//         // let accepted = confirmations.unwrap().accepted.unwrap_or(0) + 1;

//         let scores = sqlx::query_as!(
//             TeamScore,
//             "SELECT s1.score as score_1, s2.score as score_2
//             FROM game g
//             INNER JOIN score s1 ON s1.game_id = g.id AND s1.team_id = $1
//             INNER JOIN score s2 ON s2.game_id = g.id AND s2.team_id = $2 AND s1.round = s2.round
//             WHERE g.id = $3",
//             game.team_id_1,
//             game.team_id_2,
//             game.id
//         )
//         .fetch_all(&data.db)
//         .await
//         .unwrap();

//         let game_data = GameData {
//             id: game.id,
//             team_1_usernames: usernames_team_1
//                 .iter()
//                 .map(|u| UserDetails {
//                     username: u.username.clone(),
//                     profile_picture: u.profile_picture.clone(),
//                 })
//                 .collect(),
//             team_2_usernames: usernames_team_2
//                 .iter()
//                 .map(|u| UserDetails {
//                     username: u.username.clone(),
//                     profile_picture: u.profile_picture.clone(),
//                 })
//                 .collect(),
//             reporter_username: reporter.username,
//             total: total,
//             players: players,
//             accepted: accepted,
//             status: game.status,
//             scores: scores,
//             created_at: game.created_at,
//         };
//         games_data.push(game_data);
//     }

//     Ok((
//         StatusCode::OK,
//         Json(json!({"status": "success","data": json!({"games": games_data})})),
//     ))
// }

pub async fn reportable_sessions(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetReportableSessions>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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
         AND DATE(ses.end_time) > DATE(NOW() - INTERVAL '3 days')
         AND ses.end_time < NOW()
         ORDER BY ses.start_time ASC",
         body.user_id
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
