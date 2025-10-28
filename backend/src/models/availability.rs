use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, postgres::PgRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AvailabilitySlot {
    pub id: Uuid,
    pub mua_id: Uuid,
    pub start_time: String, // HH:MM format
    pub end_time: String,   // HH:MM format
    pub day_of_week: Option<i32>, // 0-6, Sunday to Saturday, NULL for specific dates
    pub specific_date: Option<DateTime<Utc>>, // For one-time availability
    pub is_available: bool,
    pub recurring: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAvailabilityRequest {
    pub start_time: String,
    pub end_time: String,
    pub recurring: bool,
    pub day_of_week: Option<Vec<i32>>,
    pub specific_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAvailabilityRequest {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub is_available: Option<bool>,
    pub recurring: Option<bool>,
    pub day_of_week: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct AvailabilityResponse {
    pub id: Uuid,
    pub mua_id: Uuid,
    pub start_time: String,
    pub end_time: String,
    pub day_of_week: Option<i32>,
    pub specific_date: Option<DateTime<Utc>>,
    pub is_available: bool,
    pub recurring: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Simplified response for frontend TimeSlot interface
#[derive(Debug, Serialize)]
pub struct TimeSlotResponse {
    pub id: String,
    pub start_time: String,
    pub end_time: String,
    pub is_available: bool,
    pub recurring: bool,
    pub day_of_week: Option<i32>,
    pub specific_date: Option<String>,
}

// Calendar booking response with customer info
#[derive(Debug, Serialize)]
pub struct CalendarBooking {
    pub id: Uuid,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub service_type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: String,
    pub location: Option<String>,
    pub notes: Option<String>,
    pub price: f64,
}

impl TryFrom<PgRow> for AvailabilitySlot {
    type Error = anyhow::Error;

    fn try_from(row: PgRow) -> Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            mua_id: row.try_get("mua_id")?,
            start_time: row.try_get("start_time")?,
            end_time: row.try_get("end_time")?,
            day_of_week: row.try_get("day_of_week")?,
            specific_date: row.try_get("specific_date")?,
            is_available: row.try_get("is_available")?,
            recurring: row.try_get("recurring")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}