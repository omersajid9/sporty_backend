use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPlayer {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignIn {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUp {
    pub username: String,
    pub password: String,
    pub profile_picture: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditPlayer {
    pub user_id: Uuid,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeletePlayer {
    pub user_id: Uuid,
}
