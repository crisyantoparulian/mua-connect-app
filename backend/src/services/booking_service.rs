use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;

use crate::models::{Booking, BookingResponse, CreateBookingRequest, UpdateBookingStatusRequest, BookingStatus};
use crate::repository::traits::{UserRepository, MuaRepository, BookingRepository};
use super::traits::BookingService;

pub struct BookingServiceImpl {
    user_repository: Box<dyn UserRepository>,
    mua_repository: Box<dyn MuaRepository>,
    booking_repository: Box<dyn BookingRepository>,
}

impl BookingServiceImpl {
    pub fn new(
        user_repository: Box<dyn UserRepository>,
        mua_repository: Box<dyn MuaRepository>,
        booking_repository: Box<dyn BookingRepository>,
    ) -> Self {
        Self { user_repository, mua_repository, booking_repository }
    }
}

#[async_trait]
impl BookingService for BookingServiceImpl {
    async fn create_booking(
        &self,
        pool: &PgPool,
        auth_header: Option<String>,
        booking_data: CreateBookingRequest,
    ) -> Result<BookingResponse> {
        let customer_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        let booking = self.booking_repository.create_booking(pool, &booking_data, customer_id).await?;

        Ok(BookingResponse {
            id: booking.id,
            customer_id: booking.customer_id,
            mua_id: booking.mua_id,
            service_type: booking.service_type,
            description: booking.description,
            event_date: booking.event_date,
            event_location: booking.event_location,
            duration_hours: booking.duration_hours,
            price: booking.price,
            status: booking.status,
            deposit_amount: booking.deposit_amount,
            deposit_paid: booking.deposit_paid,
            final_payment_paid: booking.final_payment_paid,
            created_at: booking.created_at,
            updated_at: booking.updated_at,
        })
    }

    async fn get_user_bookings(&self, pool: &PgPool, auth_header: Option<String>) -> Result<Vec<BookingResponse>> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Check if user is MUA or customer
        let user_type = self.user_repository.get_user_type(pool, user_id).await?;

        let bookings = if user_type == "mua" {
            // Get MUA profile first
            let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
                .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

            self.booking_repository.find_bookings_by_mua(pool, mua_id).await?
        } else {
            self.booking_repository.find_bookings_by_customer(pool, user_id).await?
        };

        Ok(bookings.into_iter().map(|b: Booking| BookingResponse {
            id: b.id,
            customer_id: b.customer_id,
            mua_id: b.mua_id,
            service_type: b.service_type,
            description: b.description,
            event_date: b.event_date,
            event_location: b.event_location,
            duration_hours: b.duration_hours,
            price: b.price,
            status: b.status,
            deposit_amount: b.deposit_amount,
            deposit_paid: b.deposit_paid,
            final_payment_paid: b.final_payment_paid,
            created_at: b.created_at,
            updated_at: b.updated_at,
        }).collect())
    }

    async fn update_booking_status(
        &self,
        pool: &PgPool,
        auth_header: Option<String>,
        booking_id: Uuid,
        status_data: UpdateBookingStatusRequest,
    ) -> Result<BookingResponse> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Check if user has permission to update this booking
        let booking = self.booking_repository.find_booking_by_id(pool, booking_id).await?
            .ok_or_else(|| anyhow::anyhow!("Booking not found"))?;

        // Check if user is either the customer or the MUA
        if booking.customer_id != user_id {
            // If not customer, check if user is the MUA
            let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
                .ok_or_else(|| anyhow::anyhow!("Unauthorized"))?;

            if booking.mua_id != mua_id {
                return Err(anyhow::anyhow!("Unauthorized"));
            }
        }

        let updated_booking = self.booking_repository.update_booking_status(
            pool,
            booking_id,
            status_data.status
        ).await?;

        Ok(BookingResponse {
            id: updated_booking.id,
            customer_id: updated_booking.customer_id,
            mua_id: updated_booking.mua_id,
            service_type: updated_booking.service_type,
            description: updated_booking.description,
            event_date: updated_booking.event_date,
            event_location: updated_booking.event_location,
            duration_hours: updated_booking.duration_hours,
            price: updated_booking.price,
            status: updated_booking.status,
            deposit_amount: updated_booking.deposit_amount,
            deposit_paid: updated_booking.deposit_paid,
            final_payment_paid: updated_booking.final_payment_paid,
            created_at: updated_booking.created_at,
            updated_at: updated_booking.updated_at,
        })
    }
}

// Legacy functions for backward compatibility
pub async fn create_booking(
    pool: &PgPool,
    auth_header: Option<String>,
    booking_data: CreateBookingRequest,
) -> Result<BookingResponse> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let booking_service = BookingServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );
    booking_service.create_booking(pool, auth_header, booking_data).await
}

pub async fn get_user_bookings(pool: &PgPool, auth_header: Option<String>) -> Result<Vec<BookingResponse>> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let booking_service = BookingServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );
    booking_service.get_user_bookings(pool, auth_header).await
}

pub async fn update_booking_status(
    pool: &PgPool,
    auth_header: Option<String>,
    booking_id: Uuid,
    status_data: UpdateBookingStatusRequest,
) -> Result<BookingResponse> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let booking_service = BookingServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );
    booking_service.update_booking_status(pool, auth_header, booking_id, status_data).await
}