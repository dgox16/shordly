use axum::{routing::get, Router};

use crate::handlers::checkhealh_handlers::checkhealth_handler;

pub fn checkhealth_router() -> Router {
    Router::new().route("/api/checkhealth", get(checkhealth_handler))
}
