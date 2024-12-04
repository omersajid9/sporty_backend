use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage};
use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use uuid::Uuid;

use crate::{
    model::{
        game::Game,
        notifications::{Notification, NotificationToken},
        player::Player,
        session::{Session, SessionRsvp},
    },
    schema::notifications::{GetNotification, PostNotificationToken},
    AppState,
};
// async fn send_push_notifications(push_tokens: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
async fn send_push_notifications(
    push_tokens: Vec<String>,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let expo = Expo::new(ExpoClientOptions { access_token: None });
    if push_tokens.is_empty() {
        return Ok(());
    }
    let expo_push_message = ExpoPushMessage::builder(push_tokens)
        .body(message)
        .build()?;
    let tickets = expo.send_push_notifications(expo_push_message).await;

    match tickets {
        Ok(receipts) => println!("PUSH NOTIFICATIONS TICKETS {:?}", receipts),
        Err(e) => eprintln!("Failed to send push notifications: {:?}", e),
    }
    Ok(())
}

pub async fn session_rsvp_channel(data: Arc<AppState>, rsvp: SessionRsvp) {
    // get players currently going to the session except for the user id

    let session = sqlx::query_as!(
        Session,
        "SELECT * FROM session WHERE id = $1",
        rsvp.session_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let rsvp_player = sqlx::query_as!(Player, "SELECT * FROM player WHERE id = $1", rsvp.player_id)
        .fetch_one(&data.db)
        .await
        .unwrap();

    let mut players = sqlx::query_as!(
        Player,
        "SELECT p.*
        FROM session_rsvp sr
        INNER JOIN player p ON p.id = sr.player_id
        WHERE sr.session_id = $1 AND sr.player_id != $2 AND sr.player_rsvp = 'Yes'",
        rsvp.session_id,
        rsvp.player_id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let host = sqlx::query_as!(
        Player,
        "SELECT p.*
        FROM session s
        INNER JOIN player p ON p.id = s.host_id
        WHERE s.id = $1",
        rsvp.session_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    players.push(host);

    for player in &players {
        let _ = sqlx::query_as!(
            Notification,
            "INSERT INTO 
            notification (player_id, channel, message)
            VALUES ($1, 'session_rsvp', $2)",
            player.id,
            format!(
                "{} has RSVP'd to your session {}",
                rsvp_player.username, session.session_name
            )
        )
        .execute(&data.db)
        .await
        .unwrap();
    }

    let push_tokens = sqlx::query_as!(
        NotificationToken,
        "SELECT * FROM notification_token WHERE player_id = ANY($1)",
        &players.iter().map(|p| p.id).collect::<Vec<Uuid>>()
    )
    .fetch_all(&data.db)
    .await
    .unwrap()
    .iter()
    .map(|t| t.token.clone())
    .collect::<Vec<String>>();

    send_push_notifications(
        push_tokens,
        format!(
            "{} has RSVP'd to your session {}",
            rsvp_player.username, session.session_name
        )
        .as_str(),
    )
    .await
    .unwrap();
    // println!("PUSH NOTIFICATIONS TICKETS {:?}", tickets);
    // add the data about the notification to notification data

    // use notifications token table to retrieve token of users
    // send notifications to users
    // println!("🔔 New session RSVP in handler: {:?}", data);
}

pub async fn game_channel(data: Arc<AppState>, game: Game) {
    // get players currently going to the session except for the user id
    let reporter = sqlx::query_as!(
        Player,
        "SELECT * FROM player WHERE id = $1",
        game.reporter_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let players = sqlx::query_as!(
        Player,
        "SELECT p.*
        FROM team_member tm
        INNER JOIN player p ON p.id = tm.player_id
        WHERE p.id != $1 AND tm.team_id = $2 OR tm.team_id = $3",
        game.reporter_id,
        game.team_id_1,
        game.team_id_2
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    for player in &players {
        let _ = sqlx::query_as!(
            Notification,
            "INSERT INTO 
            notification (player_id, channel, message)
            VALUES ($1, 'game', $2)",
            player.id,
            format!(
                "{} has reported scores to your session.",
                reporter.username
            )
        )
        .execute(&data.db)
        .await
        .unwrap();
    }

    let push_tokens = sqlx::query_as!(
        NotificationToken,
        "SELECT * FROM notification_token WHERE player_id = ANY($1)",
        &players.iter().map(|p| p.id).collect::<Vec<Uuid>>()
    )
    .fetch_all(&data.db)
    .await
    .unwrap()
    .iter()
    .map(|t| t.token.clone())
    .collect::<Vec<String>>();

    send_push_notifications(
        push_tokens,
        format!(
            "{} has reported scores to your session.",
            reporter.username
        )
        .as_str(),
    ).await
    .unwrap();
}


pub async fn game_status_change_channel(data: Arc<AppState>, game: Game) {
    let session = sqlx::query_as!(
        Session,
        "SELECT * FROM session WHERE id = $1",
        game.session_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let players = sqlx::query_as!(
        Player,
        "SELECT p.*
        FROM team_member tm
        INNER JOIN player p ON p.id = tm.player_id
        WHERE tm.team_id = $1 OR tm.team_id = $2",
        game.team_id_1,
        game.team_id_2
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    for player in &players {
        let _ = sqlx::query_as!(
            Notification,
            "INSERT INTO 
            notification (player_id, channel, message)
            VALUES ($1, 'game_status_update', $2)",
            player.id,
            format!(
                "Your game score for session {} has been {}.",
                session.session_name,
                if game.status == "Yes" {"confirmed"} else {"rejected"}
            )
        )
        .execute(&data.db)
        .await
        .unwrap();
    }

    let push_tokens = sqlx::query_as!(
        NotificationToken,
        "SELECT * FROM notification_token WHERE player_id = ANY($1)",
        &players.iter().map(|p| p.id).collect::<Vec<Uuid>>()
    )
    .fetch_all(&data.db)
    .await
    .unwrap()
    .iter()
    .map(|t| t.token.clone())
    .collect::<Vec<String>>();

    send_push_notifications(
        push_tokens,
        format!(
            "Your game score for session {} has been {}.",
            session.session_name,
            if game.status == "Yes" {"confirmed"} else {"rejected"}
        )
        .as_str(),
    ).await
    .unwrap();
}

pub async fn get_notifications(
    State(data): State<Arc<AppState>>,
    axum::extract::Query(body): axum::extract::Query<GetNotification>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let notifications = sqlx::query_as!(
        Notification,
        "SELECT * FROM notification WHERE player_id = $1 ORDER BY created_at DESC",
        body.user_id
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "data": json!({"notifications": notifications})})),
    ))
}

pub async fn save_notification_token(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<PostNotificationToken>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query_as!(
        NotificationToken,
        "DELETE FROM
        notification_token 
        WHERE token = $1",
        body.token
    )
    .execute(&data.db)
    .await
    .unwrap();

    let _ = sqlx::query_as!(
        NotificationToken,
        "INSERT INTO
        notification_token (player_id, token)
        VALUES ($1, $2)
        ON CONFLICT (player_id, token) DO NOTHING",
        body.user_id,
        body.token
    )
    .execute(&data.db)
    .await
    .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "message": "Notification token saved successfully"})),
    ))
}
pub async fn remove_notification_token(
    State(data): State<Arc<AppState>>,
    axum::extract::Json(body): axum::extract::Json<PostNotificationToken>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query_as!(
        NotificationToken,
        "DELETE FROM
        notification_token 
        WHERE player_id = $1 AND token = $2",
        body.user_id,
        body.token
    )
    .execute(&data.db)
    .await
    .unwrap();

    Ok((
        StatusCode::OK,
        Json(json!({"status": "success", "message": "Notification token saved successfully"})),
    ))
}
