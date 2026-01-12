use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Notification {
    pub id: Uuid,
    pub player_id: Uuid,
    pub channel: String,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NotificationToken {
    pub player_id: Uuid,
    pub token: String,
    pub created_at: chrono::NaiveDateTime,
}
