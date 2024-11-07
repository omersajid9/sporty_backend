use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPlayer {
    pub username: String,
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
    pub profile_picture: String
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
