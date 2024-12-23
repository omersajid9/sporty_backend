use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use uuid::Uuid;

use crate::{
    auth::Claims,
    model::{
        game::{Game, GameData, TeamScore, UserDetails}, player::{Player, RatingData}, search::Sport, Username, ID
    },
    schema::player::*,
    AppState,
};

// pub async fn sign_in(
//     State(data): State<Arc<AppState>>,
//     axum::extract::Json(body): axum::extract::Json<SignIn>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let result = sqlx::query_as!(
//         Player,
//         "SELECT *
//         FROM player
//         WHERE username = $1 AND password = $2",
//         body.username.to_string(),
//         body.password.to_string(),
//     )
//     .fetch_one(&data.db)
//     .await;

//     match result {
//         Ok(player) => {
//             let claims = Claims::new(player.id, 24 * 3600); // Token valid for 24 hours
//             let token = claims.generate_token()
//                 .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();
//             let user_data = json!({
//                 "token": token,
//             });

//             let player_response = json!({
//                 "status": "success",
//                 "message": "player signed in successfully",
//                 "auth_token": user_data
//             });
//             Ok((StatusCode::OK, Json(player_response)))
//         }
//         Err(e) => {
//             if let sqlx::Error::RowNotFound = e {
//                 let error_response = json!({
//                     "status": "error",
//                     "message": "invalid username or password"
//                 });
//                 return Err((StatusCode::BAD_REQUEST, Json(error_response)));
//             }
//             let error_response = json!({"status": "error", "message": "an error occurred"});
//             Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
//         }
//     }
// }

// pub async fn sign_up(
//     State(data): State<Arc<AppState>>,
//     axum::extract::Json(body): axum::extract::Json<SignUp>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let result = sqlx::query_as!(
//         Player,
//         "INSERT INTO player
//             (username, password, profile_picture)
//             VALUES ($1, $2, $3)
//             RETURNING *",
//         body.username.to_string(),
//         body.password.to_string(),
//         body.profile_picture
//     )
//     .fetch_one(&data.db)
//     .await;

//     match result {
//         Ok(player) => {
//             let claims = Claims::new(player.id, 24 * 3600); // Token valid for 24 hours
//             let token = claims.generate_token()
//                 .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();
//             let user_data = json!({
//                 "token": token,
//             });

//             let player_response =
//                 json!({"status": "success", "auth_token": user_data});
//             Ok((StatusCode::CREATED, Json(player_response)))
//         }
//         Err(e) => {
//             if let sqlx::Error::Database(db_err) = &e {
//                 if db_err.constraint() == Some("player_username_key") {
//                     let error_response =
//                         json!({"status": "error", "message": "username already taken"});
//                     return Err((StatusCode::CONFLICT, Json(error_response)));
//                 }
//             }
//             let error_response = json!({"status": "error", "message": "an error occurred"});
//             Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
//         }
//     }
// }

pub async fn delete_player(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<DeletePlayer>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query_as!(
        Player,
        "DELETE FROM player WHERE id = $1",
        body.user_id.clone()
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Player with username: {} not found", body.user_id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::OK)
}

pub async fn edit_player(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<EditPlayer>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    if let Some(username) = body.username {
        let res = sqlx::query_as!(
            Player,
            "UPDATE player SET username = $1 WHERE id = $2",
            username,
            body.user_id
        )
        .execute(&data.db)
        .await;

        if res.is_err() {
            let error_response = json!({"status": "error", "message": "username already taken"});
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }
    if let Some(password) = body.password {
        let _ = sqlx::query_as!(
            Player,
            "UPDATE player SET password = $1 WHERE id = $2",
            password,
            body.user_id
        )
        .execute(&data.db)
        .await;
    }
    if let Some(profile_picture) = body.profile_picture {
        let _ = sqlx::query_as!(
            Player,
            "UPDATE player SET profile_picture = $1 WHERE id = $2",
            profile_picture,
            body.user_id
        )
        .execute(&data.db)
        .await;
    }

    let player = sqlx::query_as!(
        Player,
        "SELECT * FROM player where id = $1",
        body.user_id
    ).fetch_one(&data.db)
    .await
    .unwrap();

    // if let username =
    let player_response = json!({
        "status": "success",
        "message": "player signed in successfully",
        "user": player
    });

    Ok((StatusCode::OK, Json(player_response)))
}

pub async fn get_player(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetPlayer>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let player = sqlx::query_as!(
        Player,
        "Select * FROM player where id = $1",
        body.user_id.clone()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let ratings = sqlx::query_as!(
        RatingData,
        "SELECT 
        r.uncertainity, 
        r.rating, 
        r.updated,
        r.mode,
        s.name as sport, 
        s.icon as sport_icon, 
        s.icon_source as sport_icon_source
        FROM rating r
        INNER JOIN sport s ON r.sport_id = s.id
        WHERE r.player_id = $1
        ORDER BY s.name DESC, r.mode DESC",
        body.user_id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let games = sqlx::query_as!(
        Game,
        "SELECT *
        FROM game g
        WHERE g.team_id_1 IN (SELECT team_id FROM team_member WHERE player_id = $1) OR g.team_id_2 IN (SELECT team_id FROM team_member WHERE player_id = $1)
        ORDER BY g.created_at DESC",
        body.user_id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();


let mut games_data: Vec<GameData> = Vec::new();

for game in games {
        let sport = sqlx::query_as!(
            Sport,
            "SELECT * 
            FROM sport
            WHERE id = (SELECT sport_id FROM session WHERE id = $1)",
            game.session_id
        ).fetch_one(&data.db)
        .await
        .unwrap();

    
        let users_team_1 = sqlx::query_as!(
            UserDetails,
            "SELECT p.username, p.id, p.profile_picture
            FROM team_member tm
            INNER JOIN player p ON p.id = tm.player_id
            WHERE tm.team_id = $1",
            game.team_id_1
        )
        .fetch_all(&data.db)
        .await
        .unwrap();

        let users_team_2 = sqlx::query_as!(
            UserDetails,
            "SELECT p.username, p.id, p.profile_picture
            FROM team_member tm
            INNER JOIN player p ON p.id = tm.player_id
            WHERE tm.team_id = $1",
            game.team_id_2
        )
        .fetch_all(&data.db)
        .await
        .unwrap();

        if users_team_2.len() < 1 || users_team_1.len() < 1 {
            continue;
        }

        let reporter = sqlx::query_as!(
            UserDetails,
            "SELECT p.username, p.id, p.profile_picture
             FROM player p
             WHERE id = $1",
            game.reporter_id
        )
        .fetch_one(&data.db)
        .await
        .unwrap();

        let user_ids_1: Vec<Uuid> = users_team_1.iter().map(|u| u.id.clone()).collect();
        let user_ids_2: Vec<Uuid> = users_team_1.iter().map(|u| u.id.clone()).collect();
        let mut players = Vec::new();
        players.extend(user_ids_1);
        players.extend(user_ids_2);

        let accepted: Vec<Uuid> = sqlx::query_as!(
            ID,
            "SELECT p.id
                FROM player p
            INNER JOIN score_validation sv ON sv.player_id = p.id
            WHERE sv.game_id = $1
            AND sv.status = 'Yes'",
            game.id
        )
        .fetch_all(&data.db)
        .await
        .unwrap()
        .iter()
        .map(|u| u.id.clone())
        .collect();

        let total: Vec<Uuid> = sqlx::query_as!(
            ID,
            "SELECT p.id
                FROM player p
            INNER JOIN score_validation sv ON sv.player_id = p.id
            WHERE sv.game_id = $1",
            game.id
        )
        .fetch_all(&data.db)
        .await
        .unwrap()
        .iter()
        .map(|u| u.id.clone())
        .collect();

        // let total: i64 = confirmations.unwrap().total.unwrap_or(0) + 1;
        // let accepted = confirmations.unwrap().accepted.unwrap_or(0) + 1;

        let scores = sqlx::query_as!(
            TeamScore,
            "SELECT s1.score as score_1, s2.score as score_2
            FROM game g
            INNER JOIN score s1 ON s1.game_id = g.id AND s1.team_id = $1
            INNER JOIN score s2 ON s2.game_id = g.id AND s2.team_id = $2 AND s1.round = s2.round
            WHERE g.id = $3",
            game.team_id_1,
            game.team_id_2,
            game.id
        )
        .fetch_all(&data.db)
        .await
        .unwrap();

        let game_data = GameData {
            id: game.id,
            team_1_users: users_team_1,
            team_2_users: users_team_2,
            reporter_user: reporter,
            sport: sport,
            total: total,
            players: players,
            accepted: accepted,
            status: game.status,
            scores: scores,
            created_at: game.created_at,
        };
        games_data.push(game_data);
    }

    Ok((
        StatusCode::OK,
        Json(
            json!({"status": "success", "data": json!({"profile": player, "games": games_data, "ratings": ratings})}),
        ),
    ))
}
