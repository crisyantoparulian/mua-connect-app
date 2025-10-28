use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub id: Uuid,
    pub booking_id: Uuid,
    pub reviewer_id: Uuid,
    pub reviewee_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub booking_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
}