use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Serialize, Deserialize, Debug)]
pub struct GetPlayers{
    pub query: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct GetExploreSessions {
    pub user_id: Option<Uuid>,
    pub sport: String,
    pub date: Option<chrono::NaiveDate>,
    pub lat: f64,
    pub lng: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGoingSessions {
    pub user_id: Uuid,
    pub lat: f64,
    pub lng: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetReportableSessions {
    pub user_id: Uuid
}
