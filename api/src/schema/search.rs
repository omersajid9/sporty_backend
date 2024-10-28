use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct GetPlayers{
    pub query: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct GetExploreSessions {
    pub username: String,
    pub sport: String,
    pub date: Option<chrono::NaiveDate>,
    pub lat: f64,
    pub lng: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetGoingSessions {
    pub username: String,
    pub lat: f64,
    pub lng: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetReportableSessions {
    pub username: String
}
