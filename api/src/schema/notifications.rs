use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotification {
    pub username: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PostNotificationToken {
    pub username: String,
    pub token: String
}