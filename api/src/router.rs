use std::sync::Arc;

use axum::Router;

use crate::{
    models::utils_models::AppState,
    routes::{checkhealth_routes::checkhealth_router, url_routes::url_router},
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(checkhealth_router())
        .merge(url_router(app_state))
}
