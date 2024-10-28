use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Session {
    pub id: Uuid,
    pub session_name: String,
    pub sport_id: Uuid,
    pub host_id: Uuid,
    pub location_name: String,
    pub lat: f64,
    pub lon: f64,
    pub public: bool,
    pub max_players: i32,
    pub time: chrono::NaiveDateTime,
}


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SessionData {
    pub id: Uuid,
    pub session_name: String,
    pub username: String,
    pub username_icon: String,
    pub sport: String,
    pub sport_icon: String,
    pub sport_icon_source: String,
    pub location_name: String,
    pub lat: f64,
    pub lon: f64,
    pub dis: Option<f64>,
    pub time: chrono::NaiveDateTime,
    pub max_players: i32,
    pub count_rsvps: Option<i64>
}


