use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;
use sqlx::PgPool;

use crate::models::{
    CreateUserRequest, LoginRequest, AuthResponse, UserResponse,
    SearchMuasRequest, MuaProfileResponse, CreateMuaProfileRequest, CreateBookingRequest,
    BookingResponse, UpdateBookingStatusRequest
};
use crate::repository::traits::{UserRepository, MuaRepository, BookingRepository};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register(&self, pool: &PgPool, req: CreateUserRequest) -> Result<AuthResponse>;
    async fn login(&self, pool: &PgPool, req: LoginRequest) -> Result<AuthResponse>;
    async fn verify_token(&self, token: &str) -> Result<Uuid>;
}

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_profile(&self, pool: &PgPool, auth_header: Option<String>) -> Result<UserResponse>;
    async fn update_profile(&self, pool: &PgPool, auth_header: Option<String>, profile_data: Value) -> Result<UserResponse>;
}

#[async_trait]
pub trait MuaService: Send + Sync {
    async fn search_muas(&self, pool: &PgPool, params: SearchMuasRequest) -> Result<Vec<MuaProfileResponse>>;
    async fn get_mua_by_id(&self, pool: &PgPool, mua_id: Uuid) -> Result<MuaProfileResponse>;
    async fn create_profile(&self, pool: &PgPool, auth_header: Option<String>, profile_data: CreateMuaProfileRequest) -> Result<MuaProfileResponse>;
    async fn create_portfolio_item(&self, pool: &PgPool, auth_header: Option<String>, portfolio_data: Value) -> Result<Value>;
}

#[async_trait]
pub trait BookingService: Send + Sync {
    async fn create_booking(
        &self,
        pool: &PgPool,
        auth_header: Option<String>,
        booking_data: CreateBookingRequest,
    ) -> Result<BookingResponse>;

    async fn get_user_bookings(&self, pool: &PgPool, auth_header: Option<String>) -> Result<Vec<BookingResponse>>;

    async fn update_booking_status(
        &self,
        pool: &PgPool,
        auth_header: Option<String>,
        booking_id: Uuid,
        status_data: UpdateBookingStatusRequest,
    ) -> Result<BookingResponse>;
}