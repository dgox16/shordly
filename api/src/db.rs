use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

pub async fn connect_db(database_url: &str) -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}
