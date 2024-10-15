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
    pub profile_picture: Option<String>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Sport {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub icon: String
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PlayerSport {
    pub id: Uuid,
    pub player_id: Uuid,
    pub sport_id: Uuid
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Rating {
    pub player_sport_id: Uuid,
    pub rating: i64,
    pub std: f64,
    pub val: f64,
    pub updated: chrono::NaiveDate,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Session {
    pub id: Uuid,
    pub sport_id: Uuid,
    pub host_id: Uuid,
    pub lat: f64,
    pub lon: f64,
    pub public: bool,
    pub max_players: i64,
    pub time: chrono::NaiveDateTime,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SessionRsvp {
    pub session_id: Uuid,
    pub player_id: Uuid,
    pub player_rsvp: String,
    pub host_rsvp: String
}


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SessionData {
    pub id: Uuid,
    pub username: String,
    pub sport: String,
    pub lat: f64,
    pub lon: f64,
    pub time: chrono::NaiveDateTime,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Count {
    pub count: Option<i64>,
}