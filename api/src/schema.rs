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
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGame {
    pub sport: String,
    pub username: String,
    pub location: Location,
    pub time: chrono::NaiveDateTime
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditGame {
    pub game_id: Uuid,
    pub username: String,
    pub location: Option<Location>,
    pub time: Option<chrono::NaiveDateTime>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteGame {
    pub game_id: Uuid,
    pub username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Rsvp {
    Maybe,
    Yes,
    No
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGamePlayer {
    pub game_id: Uuid,
    pub player_username: String,
    pub rsvp: Rsvp
}