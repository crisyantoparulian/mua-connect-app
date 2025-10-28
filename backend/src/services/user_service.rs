use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;
use sqlx::PgPool;

use crate::models::{UserResponse, User};
use crate::repository::traits::UserRepository;
use super::traits::UserService;

pub struct UserServiceImpl {
    user_repository: Box<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_profile(&self, pool: &PgPool, auth_header: Option<String>) -> Result<UserResponse> {
        let user_id = get_user_id_from_auth_header(auth_header)?;
        let user = self.user_repository.find_by_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        Ok(UserResponse::from(user))
    }

    async fn update_profile(&self, pool: &PgPool, auth_header: Option<String>, profile_data: Value) -> Result<UserResponse> {
        let user_id = get_user_id_from_auth_header(auth_header)?;
        let updated_user = self.user_repository.update_user(pool, user_id, &profile_data).await?;

        Ok(UserResponse::from(updated_user))
    }
}

// Helper function that can be used by services
pub fn get_user_id_from_auth_header(auth_header: Option<String>) -> Result<Uuid> {
    let auth_header = auth_header
        .ok_or_else(|| anyhow::anyhow!("Missing authorization header"))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(anyhow::anyhow!("Invalid authorization format"));
    }

    let token = &auth_header[7..];
    super::auth_service::verify_jwt_token(token)
}

// Legacy functions for backward compatibility
pub async fn get_user_profile(pool: &PgPool, auth_header: Option<String>) -> Result<UserResponse> {
    let user_id = get_user_id_from_auth_header(auth_header)?;
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let user = user_repository.find_by_id(pool, user_id).await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    Ok(UserResponse::from(user))
}

pub async fn update_user_profile(pool: &PgPool, auth_header: Option<String>, profile_data: Value) -> Result<UserResponse> {
    let user_id = get_user_id_from_auth_header(auth_header)?;
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let updated_user = user_repository.update_user(pool, user_id, &profile_data).await?;

    Ok(UserResponse::from(updated_user))
}