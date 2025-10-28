use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::{PgPool, query, query_as, types::BigDecimal};
use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::models::{Booking, CreateBookingRequest, UpdateBookingStatusRequest, BookingStatus};
use super::traits::BookingRepository;

#[derive(Debug, Clone)]
pub struct BookingRepositoryImpl;

impl BookingRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BookingRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BookingRepository for BookingRepositoryImpl {
    async fn create_booking(&self, pool: &PgPool, booking_data: &CreateBookingRequest, customer_id: Uuid) -> Result<Booking> {
        // Parse the event date string
        let event_date = DateTime::parse_from_rfc3339(&booking_data.event_date)
            .map_err(|_| anyhow::anyhow!("Invalid date format"))?
            .with_timezone(&Utc);

        // Parse price string to BigDecimal
        let price = BigDecimal::from_str(&booking_data.price)
            .map_err(|_| anyhow::anyhow!("Invalid price format"))?;

        // Parse deposit amount if provided
        let deposit_amount = booking_data.deposit_amount.as_ref()
            .map(|s| BigDecimal::from_str(s))
            .transpose()
            .map_err(|_| anyhow::anyhow!("Invalid deposit amount format"))?;

        let booking_row = query_as::<_, Booking>(
            r#"
            INSERT INTO bookings (
                customer_id, mua_id, service_type, description, event_date,
                event_location, duration_hours, price, status,
                deposit_amount, deposit_paid, final_payment_paid, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, NOW(), NOW())
            RETURNING id, customer_id, mua_id, service_type, description, event_date,
                      event_location, duration_hours, price, status,
                      deposit_amount, deposit_paid, final_payment_paid, created_at, updated_at
            "#
        )
        .bind(customer_id)
        .bind(booking_data.mua_id)
        .bind(&booking_data.service_type)
        .bind(booking_data.description.as_deref())
        .bind(event_date)
        .bind(&booking_data.event_location)
        .bind(booking_data.duration_hours)
        .bind(price)
        .bind(BookingStatus::Pending as BookingStatus)
        .bind(deposit_amount)
        .bind(false)
        .bind(false)
        .fetch_one(pool)
        .await?;

        Ok(booking_row)
    }

    async fn find_booking_by_id(&self, pool: &PgPool, booking_id: Uuid) -> Result<Option<Booking>> {
        let booking = query_as::<_, Booking>("SELECT * FROM bookings WHERE id = $1")
            .bind(booking_id)
            .fetch_optional(pool)
            .await?;

        Ok(booking)
    }

    async fn find_bookings_by_customer(&self, pool: &PgPool, customer_id: Uuid) -> Result<Vec<Booking>> {
        let bookings = query_as::<_, Booking>("SELECT * FROM bookings WHERE customer_id = $1 ORDER BY created_at DESC")
            .bind(customer_id)
            .fetch_all(pool)
            .await?;

        Ok(bookings)
    }

    async fn find_bookings_by_mua(&self, pool: &PgPool, mua_id: Uuid) -> Result<Vec<Booking>> {
        let bookings = query_as::<_, Booking>("SELECT * FROM bookings WHERE mua_id = $1 ORDER BY created_at DESC")
            .bind(mua_id)
            .fetch_all(pool)
            .await?;

        Ok(bookings)
    }

    async fn update_booking_status(&self, pool: &PgPool, booking_id: Uuid, status: BookingStatus) -> Result<Booking> {
        let updated_booking = query_as::<_, Booking>(
            "UPDATE bookings SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING *"
        )
        .bind(status as BookingStatus)
        .bind(booking_id)
        .fetch_one(pool)
        .await?;

        Ok(updated_booking)
    }
}