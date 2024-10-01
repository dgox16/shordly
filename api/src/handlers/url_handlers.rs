use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use serde_json::json;

use crate::{
    models::{url_models::UrlModel, utils_models::AppState},
    responses::error_responses::error_db,
    schemas::url_schemas::{CreateShortUrlSchema, UrlParams},
    utils::encode_functions::encode_id,
    validators::url_validators::{validate_existing_url, validate_url_structure},
};

pub async fn create_short_url_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateShortUrlSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    validate_url_structure(&body.original_url)?;
    let existing_url = validate_existing_url(&data, &body.original_url).await?;
    match existing_url {
        Some(url) => {
            let response = json!({
                "status": true,
                "data": url
            });
            Ok(Json(response))
        }
        None => {
            let new_url = sqlx::query_as!(
                UrlModel,
                "INSERT INTO urls
                    (original_url, counter_visits)
                VALUES ($1, $2)
                RETURNING *",
                body.original_url,
                0
            )
            .fetch_one(&data.db)
            .await
            .map_err(error_db)?;

            let new_url = sqlx::query_as!(
                UrlModel,
                "UPDATE urls 
                SET encoded_id = $2 
                WHERE id_url = $1
                RETURNING *",
                new_url.id_url,
                encode_id(new_url.id_url)
            )
            .fetch_one(&data.db)
            .await
            .map_err(error_db)?;

            let response = json!({
                "status": true,
                "data": new_url
            });
            Ok(Json(response))
        }
    }
}

pub async fn visit_url_handler(
    State(data): State<Arc<AppState>>,
    Path(params): Path<UrlParams>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let url_found = sqlx::query_as!(
        UrlModel,
        "SELECT * FROM urls 
        WHERE encoded_id = $1",
        params.encoded_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_db)?;

    let url_visited = sqlx::query_as!(
        UrlModel,
        "UPDATE urls 
        SET counter_visits = $2,
            visited_at = NOW()
        WHERE id_url = $1
        RETURNING *",
        url_found.id_url,
        url_found.counter_visits + 1
    )
    .fetch_one(&data.db)
    .await
    .map_err(error_db)?;

    Ok(Redirect::to(&url_visited.original_url))
}
