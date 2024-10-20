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
pub struct Sport {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub icon: String,
    pub icon_source: String
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PlayerSport {
    pub id: Uuid,
    pub player_id: Uuid,
    pub sport_id: Uuid
}

// #[derive(Debug, FromRow, Deserialize, Serialize)]
// #[allow(non_snake_case)]
// pub struct Rating {
//     pub player_sport_id: Uuid,
//     pub rating: i64,
//     pub std: f64,
//     pub val: f64,
//     pub updated: chrono::NaiveDate,
// }

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
pub struct SessionRsvp {
    pub session_id: Uuid,
    pub player_id: Uuid,
    pub player_rsvp: String,
    pub host_rsvp: String
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SessionRsvpUsername {
    pub username: String,
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

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Count {
    pub count: Option<i64>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Game {
    pub id: Uuid,
    pub session_id: Uuid,
    pub player_id_1: Uuid,
    pub player_id_2: Uuid,
    pub status: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Score {
    pub id: Uuid,
    pub game_id: Uuid,
    pub player_id: Uuid,
    pub score: i32,
    pub round: i32,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ScoreData {
    pub score_1: i32,
    pub score_2: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MatchUsernameData {
    pub username_1: String,
    pub username_2: String,
}


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameData {
    pub id: Uuid,
    pub player_id_1: Uuid,
    pub player_id_2: Uuid,
    pub username_1: String,
    pub username_2: String,
    //SPORTS INFO
    pub status: String,
    pub scores: Vec<ScoreData>,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Rating {
    pub player_id: Uuid,
    pub sport_id: Uuid,
    pub rating: f64,
    pub std: f64,
    pub val: f64,
    pub updated: chrono::NaiveDateTime
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct RatingData {
    pub sport: String,
    pub sport_icon: String,
    pub sport_icon_source: String,
    pub rating: f64,
    pub std: f64,
    pub val: f64,
    pub updated: chrono::NaiveDateTime
}


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameUsername {
    pub id: Uuid,
    pub session_id: Uuid,
    pub player_id_1: Uuid,
    pub player_id_2: Uuid,
    pub username_1: String,
    pub username_2: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime
}





// pub id: Uuid,
// pub session_name: String,
// pub username: String,
// pub username_icon: String,
// pub sport: String,
// pub sport_icon: String,
// pub lat: f64,
// pub lon: f64,
// pub time: chrono::NaiveDateTime,
// pub max_players: i32,
// pub count_rsvps: Option<i64>
