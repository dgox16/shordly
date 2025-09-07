use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::url_handlers::{create_short_url_handler, visit_url_handler},
    models::utils_models::AppState,
};

pub fn url_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/url/create", post(create_short_url_handler))
        .route("/{encoded_id}", get(visit_url_handler))
        .with_state(app_state)
}
