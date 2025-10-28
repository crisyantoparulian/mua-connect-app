// Example of how to use the service container with dependency injection

use std::sync::Arc;
use sqlx::PgPool;
use crate::services::{ServiceContainer, traits::*};
use crate::repository::{UserRepositoryImpl, MuaRepositoryImpl, BookingRepositoryImpl};

// Example 1: Using the default service container
pub async fn example_default_container(pool: &PgPool) -> anyhow::Result<()> {
    let container = ServiceContainer::new();

    // Use auth service
    let auth_result = container.auth_service.register(
        pool,
        crate::models::CreateUserRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            user_type: crate::models::UserType::Customer,
            full_name: "Test User".to_string(),
            phone_number: Some("1234567890".to_string()),
        }
    ).await?;

    println!("User registered: {}", auth_result.user.email);

    Ok(())
}

// Example 2: Using the service container with custom repositories
pub async fn example_custom_repositories(pool: &PgPool) -> anyhow::Result<()> {
    // Create custom repositories (can be mocked in tests)
    let user_repository = Box::new(UserRepositoryImpl::new());
    let mua_repository = Box::new(MuaRepositoryImpl::new());
    let booking_repository = Box::new(BookingRepositoryImpl::new());

    // Create services with custom repositories
    let auth_service = Arc::new(
        crate::services::auth_service::AuthServiceImpl::new(user_repository)
    );

    let user_service = Arc::new(
        crate::services::user_service::UserServiceImpl::new(
            Box::new(UserRepositoryImpl::new())
        )
    );

    let mua_service = Arc::new(
        crate::services::mua_service::MuaServiceImpl::new(
            Box::new(UserRepositoryImpl::new()),
            mua_repository
        )
    );

    let booking_service = Arc::new(
        crate::services::booking_service::BookingServiceImpl::new(
            Box::new(UserRepositoryImpl::new()),
            Box::new(MuaRepositoryImpl::new()),
            booking_repository
        )
    );

    let container = ServiceContainer::with_dependencies(
        auth_service,
        user_service,
        mua_service,
        booking_service,
    );

    // Use the services
    let search_params = crate::models::SearchMuasRequest {
        location: Some("New York".to_string()),
        min_rating: Some(4.0),
        specialization: Some("bridal".to_string()),
        limit: Some(10),
        page: Some(1),
        latitude: None,
        longitude: None,
        radius: None,
        date: None,
    };

    let mua_results = container.mua_service.search_muas(pool, search_params).await?;
    println!("Found {} MUAs", mua_results.len());

    Ok(())
}

// Example 3: Testing with mock services
#[cfg(test)]
mod test_examples {
    use super::*;

    #[tokio::test]
    async fn test_with_mock_services() {
        let mock_container = crate::services::container::create_mock_container();

        // Mock search doesn't require a real database
        let search_params = crate::models::SearchMuasRequest {
            location: Some("Test Location".to_string()),
            min_rating: None,
            specialization: None,
            limit: None,
            page: None,
            latitude: None,
            longitude: None,
            radius: None,
            date: None,
        };

        // This will work without a database because it uses mock services
        // In a real test, you'd pass a mock pool or None and handle it appropriately
        println!("Test with mock services setup complete");
    }
}