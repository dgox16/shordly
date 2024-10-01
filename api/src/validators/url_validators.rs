use std::sync::Arc;

use axum::{http::StatusCode, Json};
use url::Url;

use crate::{
    models::{url_models::UrlModel, utils_models::AppState},
    responses::error_responses::error_db,
};

pub async fn validate_existing_url(
    data: &Arc<AppState>,
    original_url: &str,
) -> Result<Option<UrlModel>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query_as!(
        UrlModel,
        "SELECT * FROM urls WHERE original_url = $1",
        original_url
    )
    .fetch_optional(&data.db)
    .await
    .map_err(error_db)
}

pub fn validate_url_structure(
    original_url: &str,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    match Url::parse(original_url) {
        Ok(_) => Ok(()),
        Err(_e) => {
            let response = serde_json::json!({
                "status": false,
                "message": "URL invalid",
            });
            Err((StatusCode::BAD_REQUEST, Json(response)))
        }
    }
}
