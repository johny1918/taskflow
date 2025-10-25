use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use serde_json::json;

#[allow(dead_code)]
#[derive(Serialize)]
pub enum AppError {
    DatabaseError(String),
    InvalidInput(String),
    NotFound(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            match self {
                AppError::DatabaseError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                AppError::InvalidInput(_) => axum::http::StatusCode::BAD_REQUEST,
                AppError::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            },
            Json(json!({ "error": self.to_string() })),
        )
            .into_response()
    }
}
