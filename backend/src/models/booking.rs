use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use sqlx::{FromRow, Row, postgres::PgRow, types::BigDecimal};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;
use super::user::UserType;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Booking {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub mua_id: Uuid,
    pub service_type: String,
    pub description: Option<String>,
    pub event_date: DateTime<Utc>,
    pub event_location: String,
    pub duration_hours: i32,
    #[serde_as(as = "DisplayFromStr")]
    pub price: BigDecimal,
    pub status: BookingStatus,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub deposit_amount: Option<BigDecimal>,
    pub deposit_paid: bool,
    pub final_payment_paid: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "booking_status", rename_all = "lowercase")]
pub enum BookingStatus {
    Pending,
    Confirmed,
    Cancelled,
    Completed,
    NoShow,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub mua_id: Uuid,
    pub service_type: String,
    pub description: Option<String>,
    pub event_date: String,
    pub event_location: String,
    pub duration_hours: i32,
    pub price: String,
    pub deposit_amount: Option<String>,
}

#[serde_as]
#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub mua_id: Uuid,
    pub service_type: String,
    pub description: Option<String>,
    pub event_date: DateTime<Utc>,
    pub event_location: String,
    pub duration_hours: i32,
    #[serde_as(as = "DisplayFromStr")]
    pub price: BigDecimal,
    pub status: BookingStatus,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub deposit_amount: Option<BigDecimal>,
    pub deposit_paid: bool,
    pub final_payment_paid: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookingStatusRequest {
    pub status: BookingStatus,
}

impl TryFrom<PgRow> for Booking {
    type Error = anyhow::Error;

    fn try_from(row: PgRow) -> Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            customer_id: row.try_get("customer_id")?,
            mua_id: row.try_get("mua_id")?,
            service_type: row.try_get("service_type")?,
            description: row.try_get("description")?,
            event_date: row.try_get("event_date")?,
            event_location: row.try_get("event_location")?,
            duration_hours: row.try_get("duration_hours")?,
            price: row.try_get("price")?,
            status: row.try_get("status")?,
            deposit_amount: row.try_get("deposit_amount")?,
            deposit_paid: row.try_get("deposit_paid")?,
            final_payment_paid: row.try_get("final_payment_paid")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl TryFrom<PgRow> for UserType {
    type Error = anyhow::Error;

    fn try_from(row: PgRow) -> Result<Self> {
        let user_type_str: &str = row.try_get("user_type")?;
        match user_type_str {
            "customer" => Ok(UserType::Customer),
            "mua" => Ok(UserType::Mua),
            _ => Err(anyhow::anyhow!("Invalid user type: {}", user_type_str)),
        }
    }
}