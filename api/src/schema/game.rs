use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct ReportScore {
    pub session_id: Uuid,
    pub team_1_user_ids: Vec<Uuid>,
    pub team_2_user_ids: Vec<Uuid>,
    pub scores: Vec<[i32; 2]>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostReportScore {
    pub reporter_user_id: Uuid,
    pub reports: Vec<ReportScore>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostConfirmScore {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub confirmation: String
}