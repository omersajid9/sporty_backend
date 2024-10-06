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
    pub name: String
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

// create table
// if not exists player (
//     id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
//     username varchar(50) unique not null,
//     password text not null,
//     date_of_birth date,
//     profile_picture text
// );


// #[derive(Debug, FromRow, Deserialize, Serialize)]
// #[allow(non_snake_case)]
// pub struct NoteModel {
//     pub id: Uuid,
//     pub title: String,
//     pub content: String,
//     pub category: Option<String>,
//     pub published: Option<bool>,
//     #[serde(rename = "createdAt")]
//     pub created_at: Option<chrono::DateTime<chrono::Utc>>,
//     #[serde(rename = "updatedAt")]
//     pub updated_at: Option<chrono::DateTime<chrono::Utc>>
// }
