use serde::{
    Deserialize,
    Serialize
};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
    pub auth_type: String,
    pub auth_id: String,
    pub profile_picture: String
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct RatingData {
    pub sport: String,
    pub sport_icon: String,
    pub sport_icon_source: String,
    pub rating: f64,
    pub mode: String,
    pub uncertainity: f64,
    pub updated: chrono::NaiveDateTime
}

