use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod player;
pub mod search;
pub mod session;
pub mod game;
pub mod notifications;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Count {
    pub count: Option<i64>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Username {
    pub username: String
}