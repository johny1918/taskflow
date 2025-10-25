use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub done: bool,
}
