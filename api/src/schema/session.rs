use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct PostCreateSession {
    pub sport: String,
    pub session_name: String,
    pub username: String,
    pub location_name: String,
    pub lat: f64,
    pub lng: f64,
    pub time: chrono::NaiveDateTime,
    pub public: bool,
    pub max_players: i32
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PatchEditSession {
    pub session_id: Uuid,
    pub username: String,
    pub location_name: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub time: Option<chrono::NaiveDateTime>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteDeleteSession {
    pub session_id: Uuid,
    pub username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostSessionRsvp {
    pub session_id: Uuid,
    pub player_username: String,
    pub player_rsvp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSessionPlayers {
    pub session_id: Uuid,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetSession {
    pub lat: f64,
    pub lng: f64,
}
