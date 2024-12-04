use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct PostFollow {
    pub username: String,
    pub follower_username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFollow {
    pub username: String,
    pub follower_username: String
}