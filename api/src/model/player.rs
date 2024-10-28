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
    pub password: String,
    pub date_of_birth: chrono::NaiveDate,
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

