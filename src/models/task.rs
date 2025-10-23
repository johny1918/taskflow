
#[derive(sqlx::FromRow, Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}