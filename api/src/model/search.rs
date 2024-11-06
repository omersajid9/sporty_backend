use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Sport {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub icon: String,
    pub icon_source: String
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ExploreSessionData {
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
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub max_players: i32,
    pub count_rsvps: Option<i64>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GoingSessionData {
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
    pub start_time: chrono::NaiveDateTime,
    pub end_time: chrono::NaiveDateTime,
    pub max_players: i32,
    pub count_rsvps: Option<i64>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ReportableSessionData {
    pub id: Uuid,
    pub session_name: String,
    pub sport: String,
    pub sport_icon: String,
    pub sport_icon_source: String,
}