use serde::{
    Serialize,
    Deserialize
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlayer {
    pub username: String,
    pub password: String,
    pub date_of_birth: chrono::NaiveDate
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditPlayer {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletePlayer {
    pub username: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePlayerSport {
    pub username: String,
    pub sport: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletePlayerSport {
    pub username: String,
    pub sport: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSession {
    pub sport: String,
    pub username: String,
    pub location: Location,
    pub time: chrono::NaiveDateTime,
    pub public: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditSession {
    pub game_id: Uuid,
    pub username: String,
    pub location: Option<Location>,
    pub time: Option<chrono::NaiveDateTime>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteSession {
    pub game_id: Uuid,
    pub username: String
}

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "session_player_rsvp")] 
#[derive(Serialize, Deserialize)]
pub enum Rsvp {
    Pending,
    Yes,
    No
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct GetSessions {
    pub sport: String,
    pub date: Option<chrono::NaiveDate>,
    pub username: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSessionRsvp {
    pub session_id: Uuid,
    pub player_username: String,
    pub player_rsvp: Rsvp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSessionRsvps {
    pub session_id: Uuid,
    pub rsvp: Rsvp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActSessionRsvp {
    pub session_id: Uuid,
    pub player_username: String,
    pub player_rsvp: Rsvp
}
