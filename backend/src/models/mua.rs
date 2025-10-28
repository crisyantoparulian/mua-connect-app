use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::User;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MuaProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub experience_years: Option<i32>,
    pub specialization: Option<Vec<String>>,
    pub location: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_available: bool,
    pub average_rating: Option<f64>,
    pub total_reviews: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMuaProfileRequest {
    pub bio: Option<String>,
    pub experience_years: Option<i32>,
    pub specialization: Option<Vec<String>>,
    pub location: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(default)]
    pub profile_picture_base64: Option<String>,
    #[serde(default)]
    pub profile_picture_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MuaProfileResponse {
    pub id: Uuid,
    pub user: User,
    pub bio: Option<String>,
    pub experience_years: Option<i32>,
    pub specialization: Option<Vec<String>>,
    pub location: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_available: bool,
    pub average_rating: Option<f64>,
    pub total_reviews: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SearchMuasRequest {
    pub location: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius: Option<f64>,
    pub date: Option<String>,
    pub specialization: Option<String>,
    pub min_rating: Option<f64>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}