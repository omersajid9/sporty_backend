use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Follow {
    pub user_id: Uuid,
    pub follower_id: Uuid,
    pub created_at: chrono::NaiveDateTime
}