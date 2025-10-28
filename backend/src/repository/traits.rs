use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;
use serde_json::Value;
use chrono::{DateTime, Utc};

use crate::models::{
    User, MuaProfileResponse, CreateMuaProfileRequest, SearchMuasRequest,
    Booking, CreateBookingRequest, UpdateBookingStatusRequest, BookingStatus
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, pool: &PgPool, email: &str) -> Result<Option<User>>;
    async fn find_by_id(&self, pool: &PgPool, id: Uuid) -> Result<Option<User>>;
    async fn create_user(&self, pool: &PgPool, user: &User) -> Result<User>;
    async fn update_user(&self, pool: &PgPool, id: Uuid, updates: &Value) -> Result<User>;
    async fn get_user_type(&self, pool: &PgPool, id: Uuid) -> Result<String>;
}

#[async_trait]
pub trait MuaRepository: Send + Sync {
    async fn search_muas(&self, pool: &PgPool, params: &SearchMuasRequest) -> Result<Vec<MuaProfileResponse>>;
    async fn get_mua_by_id(&self, pool: &PgPool, mua_id: Uuid) -> Result<Option<MuaProfileResponse>>;
    async fn get_mua_by_user_id(&self, pool: &PgPool, user_id: Uuid) -> Result<Option<Uuid>>;
    async fn create_mua_profile(&self, pool: &PgPool, user_id: Uuid, profile_data: CreateMuaProfileRequest) -> Result<MuaProfileResponse>;
    async fn create_portfolio_item(&self, pool: &PgPool, mua_id: Uuid, portfolio_data: &Value) -> Result<Value>;
}

#[async_trait]
pub trait BookingRepository: Send + Sync {
    async fn create_booking(&self, pool: &PgPool, booking_data: &CreateBookingRequest, customer_id: Uuid) -> Result<Booking>;
    async fn find_booking_by_id(&self, pool: &PgPool, booking_id: Uuid) -> Result<Option<Booking>>;
    async fn find_bookings_by_customer(&self, pool: &PgPool, customer_id: Uuid) -> Result<Vec<Booking>>;
    async fn find_bookings_by_mua(&self, pool: &PgPool, mua_id: Uuid) -> Result<Vec<Booking>>;
    async fn update_booking_status(&self, pool: &PgPool, booking_id: Uuid, status: BookingStatus) -> Result<Booking>;
}