use axum::http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use dotenvy::dotenv;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use db::connect_db;
use models::utils_models::AppState;
use router::create_router;

mod db;
mod env_config;
mod handlers;
mod models;
mod responses;
mod router;
mod routes;
mod schemas;
mod utils;
mod validators;

use crate::env_config::EnvConfig;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let env_config = EnvConfig::init();

    let pool = connect_db(&env_config.database_url)
        .await
        .expect("🔥 Failure in connection with the database!");

    println!("✅ Database connected successfully!");

    let origins = [env_config.frontend_url.parse().unwrap()];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("🔥 Failure creating the listener!");

    println!("🚀 The server is listening in port 3000!");

    axum::serve(listener, app)
        .await
        .expect("🔥 Failure starting the server!");
}
