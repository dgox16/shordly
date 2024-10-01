use axum::{response::IntoResponse, Json};

pub async fn checkhealth_handler() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": true,
        "message": "The API works very well"
    });

    Json(json_response)
}
