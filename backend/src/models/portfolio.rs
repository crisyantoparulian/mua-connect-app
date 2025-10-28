use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PortfolioItem {
    pub id: Uuid,
    pub mua_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub service_type: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePortfolioRequest {
    pub title: String,
    pub description: Option<String>,
    pub image_url: String,
    pub service_type: Option<String>,
}