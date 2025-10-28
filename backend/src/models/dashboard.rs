use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::types::BigDecimal;
use crate::models::portfolio::{PortfolioItem, CreatePortfolioRequest};

#[serde_as]
#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_bookings: i64,
    pub pending_bookings: i64,
    pub confirmed_bookings: i64,
    pub completed_bookings: i64,
    pub total_revenue: f64,
    pub average_rating: Option<f64>,
    pub total_reviews: i32,
    pub portfolio_items: i32,
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct RecentBooking {
    pub id: Uuid,
    pub customer_name: String,
    pub service_type: String,
    pub event_date: DateTime<Utc>,
    pub status: String,
    #[serde_as(as = "DisplayFromStr")]
    pub price: BigDecimal,
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub stats: DashboardStats,
    pub recent_bookings: Vec<RecentBooking>,
    pub upcoming_bookings: Vec<RecentBooking>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct UpdateAvailabilityRequest {
    pub is_available: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMuaAvailabilityRequest {
    pub is_available: bool,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct UpdatePortfolioRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub service_type: Option<String>,
}