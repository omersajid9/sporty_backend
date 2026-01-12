use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct SendOptSms {
    pub phone: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogIn {
    pub auth_type: String,
    pub auth_id: String,
    pub passcode: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshToken {
    pub user_id: Uuid
}