use serde::{
    Serialize,
    Deserialize
};

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