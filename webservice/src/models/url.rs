use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Url(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ShortUrl {
    pub short_code: String,
    pub long_url: Url,
    pub user_id: Uuid,
    pub created_at: i64,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateUrlRequest {
    pub long_url: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CreateUrlResponse {
    pub short_code: String,
    pub short_url: String,
    pub long_url: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UrlListItem {
    pub short_code: String,
    pub long_url: String,
    pub created_at: i64,
}
