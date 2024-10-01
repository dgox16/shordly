use dotenvy::dotenv;
use std::env;

pub struct EnvConfig {
    pub database_url: String,
    pub frontend_url: String,
    pub secret_id: String,
}

impl EnvConfig {
    pub fn init() -> EnvConfig {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL is not assigned in your .env");
        let frontend_url =
            env::var("FRONTEND_URL").expect("FRONTEND_URL is not assigned in your .env");
        let secret_id = env::var("SECRET_ID").expect("SECRET_ID is not assigned in your .env");

        EnvConfig {
            database_url,
            frontend_url,
            secret_id,
        }
    }
}
