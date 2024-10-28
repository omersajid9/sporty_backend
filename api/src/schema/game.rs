use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct ReportScore {
    pub session_id: Uuid,
    pub team_1_usernames: Vec<String>,
    pub team_2_usernames: Vec<String>,
    pub scores: Vec<[i32; 2]>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostReportScore {
    pub reporter_username: String,
    pub reports: Vec<ReportScore>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostConfirmScore {
    pub username: String,
    pub game_id: Uuid,
    pub confirmation: String
}