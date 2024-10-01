use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct UrlModel {
    pub id_url: i32,
    pub original_url: String,
    pub encoded_id: Option<String>,
    pub counter_visits: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub visited_at: Option<DateTime<Utc>>,
}
