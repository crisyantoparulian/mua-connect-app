use std::sync::Arc;
use crate::services::{
    traits::{AuthService, UserService, MuaService, BookingService},
    auth_service::AuthServiceImpl,
    user_service::UserServiceImpl,
    mua_service::MuaServiceImpl,
    booking_service::BookingServiceImpl,
};
use crate::repository::{
    traits::{UserRepository, MuaRepository, BookingRepository},
    user_repository::UserRepositoryImpl,
    mua_repository::MuaRepositoryImpl,
    booking_repository::BookingRepositoryImpl,
};

#[derive(Clone)]
pub struct ServiceContainer {
    pub auth_service: Arc<dyn AuthService>,
    pub user_service: Arc<dyn UserService>,
    pub mua_service: Arc<dyn MuaService>,
    pub booking_service: Arc<dyn BookingService>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        // Create repositories
        let user_repository = Arc::new(UserRepositoryImpl::new());
        let mua_repository = Arc::new(MuaRepositoryImpl::new());
        let booking_repository = Arc::new(BookingRepositoryImpl::new());

        // Create services with their dependencies
        let auth_service = Arc::new(AuthServiceImpl::new(
            Box::new(UserRepositoryImpl::new()),
            Box::new(MuaRepositoryImpl::new())
        ));

        let user_service = Arc::new(UserServiceImpl::new({
            let repo: Box<dyn UserRepository> = Box::new(UserRepositoryImpl::new());
            repo
        }));

        let mua_service = Arc::new(MuaServiceImpl::new(
            {
                let repo: Box<dyn UserRepository> = Box::new(UserRepositoryImpl::new());
                repo
            },
            {
                let repo: Box<dyn MuaRepository> = Box::new(MuaRepositoryImpl::new());
                repo
            }
        ));

        let booking_service = Arc::new(BookingServiceImpl::new(
            {
                let repo: Box<dyn UserRepository> = Box::new(UserRepositoryImpl::new());
                repo
            },
            {
                let repo: Box<dyn MuaRepository> = Box::new(MuaRepositoryImpl::new());
                repo
            },
            {
                let repo: Box<dyn BookingRepository> = Box::new(BookingRepositoryImpl::new());
                repo
            }
        ));

        Self {
            auth_service,
            user_service,
            mua_service,
            booking_service,
        }
    }

    pub fn with_dependencies(
        auth_service: Arc<dyn AuthService>,
        user_service: Arc<dyn UserService>,
        mua_service: Arc<dyn MuaService>,
        booking_service: Arc<dyn BookingService>,
    ) -> Self {
        Self {
            auth_service,
            user_service,
            mua_service,
            booking_service,
        }
    }
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}

// Mock implementations for testing
#[cfg(test)]
mod mock_services {
    use super::*;
    use async_trait::async_trait;
    use anyhow::Result;
    use uuid::Uuid;
    use sqlx::PgPool;
    use serde_json::Value;

    use crate::models::*;

    #[derive(Debug, Clone)]
    pub struct MockAuthService;

    #[async_trait]
    impl AuthService for MockAuthService {
        async fn register(&self, _pool: &PgPool, _req: CreateUserRequest) -> Result<AuthResponse> {
            Ok(AuthResponse {
                user: UserResponse {
                    id: Uuid::new_v4(),
                    email: "test@example.com".to_string(),
                    user_type: "customer".to_string(),
                    full_name: "Test User".to_string(),
                    phone_number: Some("1234567890".to_string()),
                    profile_picture_url: None,
                    is_verified: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
                access_token: "mock_token".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: 604800,
            })
        }

        async fn login(&self, _pool: &PgPool, _req: LoginRequest) -> Result<AuthResponse> {
            Ok(AuthResponse {
                user: UserResponse {
                    id: Uuid::new_v4(),
                    email: "test@example.com".to_string(),
                    user_type: "customer".to_string(),
                    full_name: "Test User".to_string(),
                    phone_number: Some("1234567890".to_string()),
                    profile_picture_url: None,
                    is_verified: false,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
                access_token: "mock_token".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: 604800,
            })
        }

        async fn verify_token(&self, _token: &str) -> Result<Uuid> {
            Ok(Uuid::new_v4())
        }
    }

    #[derive(Debug, Clone)]
    pub struct MockUserService;

    #[async_trait]
    impl UserService for MockUserService {
        async fn get_profile(&self, _pool: &PgPool, _auth_header: Option<String>) -> Result<UserResponse> {
            Ok(UserResponse {
                id: Uuid::new_v4(),
                email: "test@example.com".to_string(),
                user_type: "customer".to_string(),
                full_name: "Test User".to_string(),
                phone_number: Some("1234567890".to_string()),
                profile_picture_url: None,
                is_verified: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        }

        async fn update_profile(&self, _pool: &PgPool, _auth_header: Option<String>, _profile_data: Value) -> Result<UserResponse> {
            Ok(UserResponse {
                id: Uuid::new_v4(),
                email: "test@example.com".to_string(),
                user_type: "customer".to_string(),
                full_name: "Updated User".to_string(),
                phone_number: Some("1234567890".to_string()),
                profile_picture_url: None,
                is_verified: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        }
    }

    #[derive(Debug, Clone)]
    pub struct MockMuaService;

    #[async_trait]
    impl MuaService for MockMuaService {
        async fn search_muas(&self, _pool: &PgPool, _params: SearchMuasRequest) -> Result<Vec<MuaProfileResponse>> {
            Ok(vec![])
        }

        async fn get_mua_by_id(&self, _pool: &PgPool, _mua_id: Uuid) -> Result<MuaProfileResponse> {
            Ok(MuaProfileResponse {
                id: Uuid::new_v4(),
                user: User {
                    id: Uuid::new_v4(),
                    email: "mua@example.com".to_string(),
                    password_hash: "hash".to_string(),
                    user_type: "mua".to_string(),
                    full_name: "MUA Artist".to_string(),
                    phone_number: Some("1234567890".to_string()),
                    profile_picture_url: None,
                    is_verified: true,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                },
                bio: Some("Professional makeup artist".to_string()),
                experience_years: Some(5),
                specialization: vec!["bridal".to_string()],
                location: Some("New York".to_string()),
                latitude: Some(40.7128),
                longitude: Some(-74.0060),
                is_available: true,
                average_rating: Some(4.5),
                total_reviews: Some(10),
                created_at: chrono::Utc::now(),
            })
        }

        async fn create_portfolio_item(&self, _pool: &PgPool, _auth_header: Option<String>, _portfolio_data: Value) -> Result<Value> {
            Ok(serde_json::json!({"id": Uuid::new_v4()}))
        }
    }

    #[derive(Debug, Clone)]
    pub struct MockBookingService;

    #[async_trait]
    impl BookingService for MockBookingService {
        async fn create_booking(&self, _pool: &PgPool, _auth_header: Option<String>, _booking_data: CreateBookingRequest) -> Result<BookingResponse> {
            Ok(BookingResponse {
                id: Uuid::new_v4(),
                customer_id: Uuid::new_v4(),
                mua_id: Uuid::new_v4(),
                service_type: "bridal".to_string(),
                description: "Bridal makeup".to_string(),
                event_date: chrono::Utc::now(),
                event_location: "Venue".to_string(),
                duration_hours: 4,
                price: 200.0,
                status: crate::models::BookingStatus::Pending,
                deposit_amount: 50.0,
                deposit_paid: false,
                final_payment_paid: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        }

        async fn get_user_bookings(&self, _pool: &PgPool, _auth_header: Option<String>) -> Result<Vec<BookingResponse>> {
            Ok(vec![])
        }

        async fn update_booking_status(&self, _pool: &PgPool, _auth_header: Option<String>, _booking_id: Uuid, _status_data: UpdateBookingStatusRequest) -> Result<BookingResponse> {
            Ok(BookingResponse {
                id: Uuid::new_v4(),
                customer_id: Uuid::new_v4(),
                mua_id: Uuid::new_v4(),
                service_type: "bridal".to_string(),
                description: "Bridal makeup".to_string(),
                event_date: chrono::Utc::now(),
                event_location: "Venue".to_string(),
                duration_hours: 4,
                price: 200.0,
                status: crate::models::BookingStatus::Confirmed,
                deposit_amount: 50.0,
                deposit_paid: false,
                final_payment_paid: false,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        }
    }

    pub fn create_mock_container() -> ServiceContainer {
        ServiceContainer::with_dependencies(
            Arc::new(MockAuthService),
            Arc::new(MockUserService),
            Arc::new(MockMuaService),
            Arc::new(MockBookingService),
        )
    }
}

#[cfg(test)]
pub use mock_services::*;