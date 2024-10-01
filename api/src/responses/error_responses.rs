use axum::{http::StatusCode, Json};
use sqlx::Error;

pub fn error_db(error: Error) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({
            "status": false,
            "message": format!("Error in the database: {}", error),
        })),
    )
}
