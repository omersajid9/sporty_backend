use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNotification {
    pub user_id: Uuid
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PostNotificationToken {
    pub user_id: Uuid,
    pub token: String
}