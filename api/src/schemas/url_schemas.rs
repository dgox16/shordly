use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateShortUrlSchema {
    pub original_url: String,
}

#[derive(Deserialize)]
pub struct UrlParams {
    pub encoded_id: String,
}
