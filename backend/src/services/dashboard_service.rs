use anyhow::Result;
use serde_json::Value;
use uuid::Uuid;
use sqlx::{PgPool, Row, types::BigDecimal};
use std::str::FromStr;

use crate::models::{
    DashboardStats, DashboardResponse, RecentBooking,
    PortfolioItem, CreatePortfolioRequest, UpdatePortfolioRequest,
    availability::{CreateAvailabilityRequest, AvailabilitySlot, AvailabilityResponse, TimeSlotResponse}
};
use chrono::NaiveTime;
use crate::models::dashboard::UpdateAvailabilityRequest;
use crate::repository::traits::{UserRepository, MuaRepository, BookingRepository};

pub struct DashboardServiceImpl {
    user_repository: Box<dyn UserRepository>,
    mua_repository: Box<dyn MuaRepository>,
    booking_repository: Box<dyn BookingRepository>,
}

impl DashboardServiceImpl {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        mua_repository: Box<dyn MuaRepository>,
        booking_repository: Box<dyn BookingRepository>,
    ) -> Self {
        Self { user_repository, mua_repository, booking_repository }
    }
}

impl DashboardServiceImpl {
    pub async fn get_dashboard(&self, pool: &PgPool, auth_header: Option<String>) -> Result<DashboardResponse> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Get MUA profile for this user
        let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

        // Get dashboard stats
        let stats = self.get_dashboard_stats(pool, mua_id).await?;

        // Get recent bookings
        let recent_bookings = self.get_recent_bookings(pool, mua_id, 5).await?;

        // Get upcoming bookings
        let upcoming_bookings = self.get_upcoming_bookings(pool, mua_id, 5).await?;

        Ok(DashboardResponse {
            stats,
            recent_bookings,
            upcoming_bookings,
        })
    }

    pub async fn update_availability(
        &self,
        pool: &PgPool,
        auth_header: Option<String>,
        request: UpdateAvailabilityRequest
    ) -> Result<Value> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Update availability in mua_profiles table
        let result = sqlx::query(
            "UPDATE mua_profiles SET is_available = $1, updated_at = NOW() WHERE user_id = $2 RETURNING is_available"
        )
        .bind(request.is_available)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(serde_json::json!({
            "is_available": result.get::<bool, _>("is_available"),
            "message": "Availability updated successfully"
        }))
    }

    pub async fn get_portfolio_items(&self, pool: &PgPool, auth_header: Option<String>) -> Result<Vec<PortfolioItem>> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Get MUA profile ID
        let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

        // Get portfolio items
        let rows = sqlx::query(
            "SELECT id, title, description, image_url, service_type, created_at FROM portfolio_items WHERE mua_id = $1 ORDER BY created_at DESC"
        )
        .bind(mua_id)
        .fetch_all(pool)
        .await?;

        let items = rows.into_iter().map(|row| PortfolioItem {
            id: row.get("id"),
            mua_id,
            title: row.get("title"),
            description: row.get("description"),
            image_url: row.get("image_url"),
            service_type: row.get("service_type"),
            created_at: row.get("created_at"),
        }).collect();

        Ok(items)
    }

    pub async fn create_portfolio_item(
        &self,
        pool: &PgPool,
        auth_header: Option<String>,
        request: CreatePortfolioRequest
    ) -> Result<PortfolioItem> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Get MUA profile ID
        let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

        // Create portfolio item
        let row = sqlx::query(
            r#"
            INSERT INTO portfolio_items (mua_id, title, description, image_url, service_type, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING id, title, description, image_url, service_type, created_at
            "#
        )
        .bind(mua_id)
        .bind(&request.title)
        .bind(&request.description)
        .bind(&request.image_url)
        .bind(&request.service_type)
        .fetch_one(pool)
        .await?;

        Ok(PortfolioItem {
            id: row.get("id"),
            mua_id,
            title: row.get("title"),
            description: row.get("description"),
            image_url: row.get("image_url"),
            service_type: row.get("service_type"),
            created_at: row.get("created_at"),
        })
    }

    pub async fn update_portfolio_item(
        &self,
        pool: &PgPool,
        auth_header: Option<String>,
        item_id: Uuid,
        request: UpdatePortfolioRequest
    ) -> Result<PortfolioItem> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Get MUA profile ID
        let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

        // Use individual UPDATE statements for each field that needs updating
        if let Some(title) = &request.title {
            sqlx::query("UPDATE portfolio_items SET title = $1, updated_at = NOW() WHERE id = $2 AND mua_id = $3")
                .bind(title)
                .bind(item_id)
                .bind(mua_id)
                .execute(pool)
                .await?;
        }

        if let Some(description) = &request.description {
            sqlx::query("UPDATE portfolio_items SET description = $1, updated_at = NOW() WHERE id = $2 AND mua_id = $3")
                .bind(description)
                .bind(item_id)
                .bind(mua_id)
                .execute(pool)
                .await?;
        }

        if let Some(image_url) = &request.image_url {
            sqlx::query("UPDATE portfolio_items SET image_url = $1, updated_at = NOW() WHERE id = $2 AND mua_id = $3")
                .bind(image_url)
                .bind(item_id)
                .bind(mua_id)
                .execute(pool)
                .await?;
        }

        if let Some(service_type) = &request.service_type {
            sqlx::query("UPDATE portfolio_items SET service_type = $1, updated_at = NOW() WHERE id = $2 AND mua_id = $3")
                .bind(service_type)
                .bind(item_id)
                .bind(mua_id)
                .execute(pool)
                .await?;
        }

        let row = sqlx::query(
            "SELECT id, title, description, image_url, service_type, created_at FROM portfolio_items WHERE id = $1 AND mua_id = $2"
        )
        .bind(item_id)
        .bind(mua_id)
        .fetch_one(pool)
        .await?;

        Ok(PortfolioItem {
            id: row.get("id"),
            mua_id,
            title: row.get("title"),
            description: row.get("description"),
            image_url: row.get("image_url"),
            service_type: row.get("service_type"),
            created_at: row.get("created_at"),
        })
    }

    pub async fn delete_portfolio_item(&self, pool: &PgPool, auth_header: Option<String>, item_id: Uuid) -> Result<Value> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Get MUA profile ID
        let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

        // Delete portfolio item
        let result = sqlx::query("DELETE FROM portfolio_items WHERE id = $1 AND mua_id = $2")
            .bind(item_id)
            .bind(mua_id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Portfolio item not found or you don't have permission to delete it"));
        }

        Ok(serde_json::json!({
            "message": "Portfolio item deleted successfully"
        }))
    }

    async fn get_dashboard_stats(&self, pool: &PgPool, mua_id: Uuid) -> Result<DashboardStats> {
        // Get booking statistics
        let booking_stats = sqlx::query!(
            r#"
            SELECT
                COUNT(*) as total_bookings,
                COUNT(CASE WHEN status = 'pending' THEN 1 END) as pending_bookings,
                COUNT(CASE WHEN status = 'confirmed' THEN 1 END) as confirmed_bookings,
                COUNT(CASE WHEN status = 'completed' THEN 1 END) as completed_bookings,
                COALESCE(SUM(CASE WHEN status = 'completed' THEN price ELSE 0 END), 0) as total_revenue
            FROM bookings
            WHERE mua_id = $1
            "#,
            mua_id
        )
        .fetch_one(pool)
        .await?;

        // Get portfolio count
        let portfolio_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM portfolio_items WHERE mua_id = $1",
            mua_id
        )
        .fetch_one(pool)
        .await?;

        // Get MUA profile info for rating
        let mua_profile = sqlx::query!(
            "SELECT average_rating, total_reviews FROM mua_profiles WHERE id = $1",
            mua_id
        )
        .fetch_one(pool)
        .await?;

        let total_revenue = booking_stats.total_revenue
            .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0))
            .unwrap_or(0.0);

        let average_rating = mua_profile.average_rating
            .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0));

        Ok(DashboardStats {
            total_bookings: booking_stats.total_bookings.unwrap_or(0),
            pending_bookings: booking_stats.pending_bookings.unwrap_or(0),
            confirmed_bookings: booking_stats.confirmed_bookings.unwrap_or(0),
            completed_bookings: booking_stats.completed_bookings.unwrap_or(0),
            total_revenue,
            average_rating,
            total_reviews: mua_profile.total_reviews.unwrap_or(0) as i32,
            portfolio_items: portfolio_count.count.unwrap_or(0) as i32,
        })
    }

    async fn get_recent_bookings(&self, pool: &PgPool, mua_id: Uuid, limit: i32) -> Result<Vec<RecentBooking>> {
        let rows = sqlx::query(
            r#"
            SELECT b.id, u.full_name as customer_name, b.service_type, b.event_date, b.status::text as status, b.price
            FROM bookings b
            JOIN users u ON b.customer_id = u.id
            WHERE b.mua_id = $1
            ORDER BY b.created_at DESC
            LIMIT $2
            "#
        )
        .bind(mua_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        let bookings = rows.into_iter().map(|row| RecentBooking {
            id: row.get("id"),
            customer_name: row.get("customer_name"),
            service_type: row.get("service_type"),
            event_date: row.get("event_date"),
            status: row.get::<String, _>("status"),
            price: row.get::<BigDecimal, _>("price"),
        }).collect();

        Ok(bookings)
    }

    async fn get_upcoming_bookings(&self, pool: &PgPool, mua_id: Uuid, limit: i32) -> Result<Vec<RecentBooking>> {
        let rows = sqlx::query(
            r#"
            SELECT b.id, u.full_name as customer_name, b.service_type, b.event_date, b.status::text as status, b.price
            FROM bookings b
            JOIN users u ON b.customer_id = u.id
            WHERE b.mua_id = $1 AND b.event_date > NOW() AND b.status IN ('pending', 'confirmed')
            ORDER BY b.event_date ASC
            LIMIT $2
            "#
        )
        .bind(mua_id)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        let bookings = rows.into_iter().map(|row| RecentBooking {
            id: row.get("id"),
            customer_name: row.get("customer_name"),
            service_type: row.get("service_type"),
            event_date: row.get("event_date"),
            status: row.get::<String, _>("status"),
            price: row.get::<BigDecimal, _>("price"),
        }).collect();

        Ok(bookings)
    }

    pub async fn create_availability_slot(
        &self,
        pool: &PgPool,
        auth_header: Option<String>,
        request: CreateAvailabilityRequest,
    ) -> Result<AvailabilityResponse> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Get MUA profile ID
        let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

        // Handle recurring slots with multiple days
        if request.recurring {
            if let Some(days) = request.day_of_week {
                let mut created_slots = Vec::new();

                for day in days {
                    let start_time = NaiveTime::parse_from_str(&request.start_time, "%H:%M")
                        .map_err(|e| anyhow::anyhow!("Invalid start_time format: {}", e))?;
                    let end_time = NaiveTime::parse_from_str(&request.end_time, "%H:%M")
                        .map_err(|e| anyhow::anyhow!("Invalid end_time format: {}", e))?;

                    let slot = sqlx::query!(
                        r#"
                        INSERT INTO availability_slots (mua_id, start_time, end_time, day_of_week, is_available, recurring, created_at, updated_at)
                        VALUES ($1, $2, $3, $4, true, true, NOW(), NOW())
                        RETURNING id, mua_id, start_time, end_time, day_of_week, specific_date, is_available, recurring, created_at, updated_at
                        "#,
                        mua_id,
                        start_time,
                        end_time,
                        day
                    )
                    .fetch_one(pool)
                    .await?;

                    created_slots.push(AvailabilityResponse {
                        id: slot.id,
                        mua_id: slot.mua_id,
                        start_time: slot.start_time.to_string(),
                        end_time: slot.end_time.to_string(),
                        day_of_week: slot.day_of_week,
                        specific_date: slot.specific_date,
                        is_available: slot.is_available,
                        recurring: slot.recurring,
                        created_at: slot.created_at.unwrap_or_else(|| chrono::Utc::now()),
                        updated_at: slot.updated_at.unwrap_or_else(|| chrono::Utc::now()),
                    });
                }

                // Return the first created slot for now (could return all if needed)
                if let Some(first_slot) = created_slots.into_iter().next() {
                    return Ok(first_slot);
                }
            } else {
                return Err(anyhow::anyhow!("Recurring slots must specify day_of_week"));
            }
        } else {
            // Handle specific date availability
            if let Some(date_str) = request.specific_date {
                // Try multiple date formats
                let specific_date = if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&date_str) {
                    dt.with_timezone(&chrono::Utc)
                } else if let Ok(date) = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                    if let Some(datetime) = date.and_hms_opt(0, 0, 0) {
                        chrono::DateTime::from_naive_utc_and_offset(datetime, chrono::Utc)
                    } else {
                        return Err(anyhow::anyhow!("Invalid date format: '{}'. Expected RFC3339, YYYY-MM-DD, or YYYY-MM-DD HH:MM:SS", date_str));
                    }
                } else if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S") {
                    chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc)
                } else {
                    return Err(anyhow::anyhow!("Invalid date format: '{}'. Expected RFC3339, YYYY-MM-DD, or YYYY-MM-DD HH:MM:SS", date_str));
                };

                let start_time = NaiveTime::parse_from_str(&request.start_time, "%H:%M")
                    .map_err(|e| anyhow::anyhow!("Invalid start_time format: {}", e))?;
                let end_time = NaiveTime::parse_from_str(&request.end_time, "%H:%M")
                    .map_err(|e| anyhow::anyhow!("Invalid end_time format: {}", e))?;

                let slot = sqlx::query!(
                    r#"
                    INSERT INTO availability_slots (mua_id, start_time, end_time, specific_date, is_available, recurring, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, true, false, NOW(), NOW())
                    RETURNING id, mua_id, start_time, end_time, day_of_week, specific_date, is_available, recurring, created_at, updated_at
                    "#,
                    mua_id,
                    start_time,
                    end_time,
                    specific_date
                )
                .fetch_one(pool)
                .await?;

                return Ok(AvailabilityResponse {
                    id: slot.id,
                    mua_id: slot.mua_id,
                    start_time: slot.start_time.to_string(),
                    end_time: slot.end_time.to_string(),
                    day_of_week: slot.day_of_week,
                    specific_date: slot.specific_date,
                    is_available: slot.is_available,
                    recurring: slot.recurring,
                    created_at: slot.created_at.unwrap_or_else(|| chrono::Utc::now()),
                    updated_at: slot.updated_at.unwrap_or_else(|| chrono::Utc::now()),
                });
            } else {
                return Err(anyhow::anyhow!("Non-recurring slots must specify specific_date"));
            }
        }

        Err(anyhow::anyhow!("Failed to create availability slot"))
    }

    pub async fn get_availability_slots(&self, pool: &PgPool, auth_header: Option<String>) -> Result<Vec<TimeSlotResponse>> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Get MUA profile ID
        let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

        // Get availability slots from database
        let rows = sqlx::query!(
            r#"
            SELECT id, mua_id, start_time, end_time, day_of_week, specific_date, is_available, recurring, created_at, updated_at
            FROM availability_slots
            WHERE mua_id = $1
            ORDER BY created_at DESC
            "#,
            mua_id
        )
        .fetch_all(pool)
        .await?;

        let slots = rows.into_iter().map(|row| TimeSlotResponse {
            id: row.id.to_string(),
            start_time: row.start_time.to_string(),
            end_time: row.end_time.to_string(),
            is_available: row.is_available,
            recurring: row.recurring,
            day_of_week: row.day_of_week,
            specific_date: row.specific_date.map(|dt| dt.to_rfc3339()),
        }).collect();

        Ok(slots)
    }

    pub async fn get_calendar_bookings(&self, pool: &PgPool, auth_header: Option<String>, start_date: &str, end_date: &str) -> Result<Vec<crate::models::availability::CalendarBooking>> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Get MUA profile ID
        let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

        // If no date range provided, get all bookings
        let date_filter = if !start_date.is_empty() && !end_date.is_empty() {
            format!("AND b.event_date >= '{}' AND b.event_date <= '{}'", start_date, end_date)
        } else {
            String::new()
        };

        let query = format!(
            r#"
            SELECT
                b.id,
                u.full_name as customer_name,
                u.phone_number as customer_phone,
                b.service_type,
                b.event_date::timestamptz as start_time,
                (b.event_date::timestamptz + b.duration_hours * interval '1 hour') as end_time,
                b.status::text as status,
                b.event_location as location,
                b.description as notes,
                b.price
            FROM bookings b
            JOIN users u ON b.customer_id = u.id
            WHERE b.mua_id = $1 {}
            ORDER BY b.event_date ASC
            "#,
            date_filter
        );

        let rows = sqlx::query(&query)
            .bind(mua_id)
            .fetch_all(pool)
            .await?;

        let bookings = rows.into_iter().map(|row| {
            let price_str: String = row.get::<BigDecimal, _>("price").to_string();
            crate::models::availability::CalendarBooking {
                id: row.get("id"),
                customer_name: row.get("customer_name"),
                customer_phone: row.get("customer_phone"),
                service_type: row.get("service_type"),
                start_time: row.get("start_time"),
                end_time: row.get("end_time"),
                status: row.get("status"),
                location: row.get("location"),
                notes: row.get("notes"),
                price: price_str.parse::<f64>().unwrap_or(0.0),
            }
        }).collect();

        Ok(bookings)
    }
}

// Legacy functions for backward compatibility
pub async fn get_dashboard(pool: &PgPool, auth_header: Option<String>) -> Result<DashboardResponse> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let dashboard_service = DashboardServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );
    dashboard_service.get_dashboard(pool, auth_header).await
}