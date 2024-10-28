use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Game {
    pub id: Uuid,
    pub session_id: Uuid,
    pub reporter_id: Uuid,
    pub team_id_1: Uuid,
    pub team_id_2: Uuid,
    pub status: String,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Score {
    pub id: Uuid,
    pub game_id: Uuid,
    pub team_id: Uuid,
    pub score: i32,
    pub round: i32,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TeamScore {
    pub score_1: i32,
    pub score_2: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserDetails {
    pub username: String,
    pub profile_picture: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameData {
    pub id: Uuid,
    pub team_1_usernames: Vec<UserDetails>,
    pub team_2_usernames: Vec<UserDetails>,
    pub reporter_username: String,
    pub scores: Vec<TeamScore>,
    pub status: String,
    pub players: Vec<String>,
    pub accepted: Vec<String>,
    pub total: Vec<String>,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ScoreConfirmation {
    pub username: String,
    pub confirmation: String
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Team {
    pub id: Uuid,
    pub name: Option<String>,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TeamMember {
    pub id: Uuid,
    pub team_id: Uuid,
    pub player_id: Uuid,
    pub created_at: chrono::NaiveDateTime
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ScoreValidation{
    pub id: Uuid,
    pub game_id: Uuid,
    pub player_id: Uuid,
    pub status: String,
    pub created_at: chrono::NaiveDateTime
}


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Rating {
    pub player_id: Uuid,
    pub sport_id: Uuid,
    pub mode: String,
    pub rating: f64,
    pub uncertainity: f64,
    pub updated: chrono::NaiveDateTime
}