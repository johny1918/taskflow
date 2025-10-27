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

#[derive(Deserialize)]
pub struct TaskFilter {
    pub done: Option<bool>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
    pub order: Option<String>,
}
