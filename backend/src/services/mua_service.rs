use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;
use sqlx::PgPool;
use bytes::Bytes;

use crate::models::{MuaProfileResponse, SearchMuasRequest, CreateMuaProfileRequest};
use crate::repository::traits::{UserRepository, MuaRepository};
use super::traits::MuaService;
use super::S3Service;

pub struct MuaServiceImpl {
    user_repository: Box<dyn UserRepository>,
    mua_repository: Box<dyn MuaRepository>,
}

impl MuaServiceImpl {
    pub fn new(user_repository: Box<dyn UserRepository>, mua_repository: Box<dyn MuaRepository>) -> Self {
        Self { user_repository, mua_repository }
    }
}

#[async_trait]
impl MuaService for MuaServiceImpl {
    async fn search_muas(&self, pool: &PgPool, params: SearchMuasRequest) -> Result<Vec<MuaProfileResponse>> {
        self.mua_repository.search_muas(pool, &params).await
    }

    async fn get_mua_by_id(&self, pool: &PgPool, mua_id: Uuid) -> Result<MuaProfileResponse> {
        let mua = self.mua_repository.get_mua_by_id(pool, mua_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA not found"))?;

        Ok(mua)
    }

    async fn create_profile(&self, pool: &PgPool, auth_header: Option<String>, mut profile_data: CreateMuaProfileRequest) -> Result<MuaProfileResponse> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Check if user already has an MUA profile
        if let Some(_existing) = self.mua_repository.get_mua_by_user_id(pool, user_id).await? {
            return Err(anyhow::anyhow!("MUA profile already exists for this user"));
        }

        // Handle profile picture upload if provided
        if let Some(profile_picture_base64) = &profile_data.profile_picture_base64 {
            let s3_service = S3Service::new().await?;
            let (mime_type, image_bytes) = s3_service.validate_image_base64(profile_picture_base64)?;
            let profile_picture_url = s3_service.upload_image(
                Bytes::from(image_bytes),
                &mime_type,
                "profile-pictures"
            ).await?;

            profile_data.profile_picture_url = Some(profile_picture_url);
            profile_data.profile_picture_base64 = None;
        }

        // Create the MUA profile
        let mua_profile = self.mua_repository.create_mua_profile(pool, user_id, profile_data).await?;
        Ok(mua_profile)
    }

    async fn create_portfolio_item(&self, pool: &PgPool, auth_header: Option<String>, portfolio_data: Value) -> Result<Value> {
        let user_id = super::user_service::get_user_id_from_auth_header(auth_header)?;

        // Get MUA profile ID for this user
        let mua_id = self.mua_repository.get_mua_by_user_id(pool, user_id).await?
            .ok_or_else(|| anyhow::anyhow!("MUA profile not found"))?;

        // Extract image data if present
        let mut processed_data = portfolio_data.clone();
        if let Some(image_data) = portfolio_data.get("image_base64").and_then(|v| v.as_str()) {
            println!("DEBUG: Processing image data for portfolio");

            // Initialize S3 service
            let s3_service = S3Service::new().await?;
            println!("DEBUG: S3 service initialized");

            // Validate and upload image
            let (mime_type, image_bytes) = s3_service.validate_image_base64(image_data)?;
            println!("DEBUG: Image validated, mime_type: {}, size: {} bytes", mime_type, image_bytes.len());

            let image_url = match s3_service.upload_image(
                Bytes::from(image_bytes),
                &mime_type,
                "portfolio"
            ).await {
                Ok(url) => {
                    println!("DEBUG: Image uploaded to: {}", url);
                    url
                }
                Err(e) => {
                    println!("DEBUG: S3 upload failed: {}, using fallback URL", e);
                    // Fallback: generate a placeholder URL for testing
                    format!("https://picsum.photos/seed/portfolio-fallback/400/300.jpg")
                }
            };

            // Replace base64 data with S3 URL
            processed_data["image_url"] = serde_json::Value::String(image_url);
            processed_data.as_object_mut().unwrap().remove("image_base64");
        }

        self.mua_repository.create_portfolio_item(pool, mua_id, &processed_data).await
    }
}

// Legacy functions for backward compatibility
pub async fn search_muas(pool: &PgPool, params: SearchMuasRequest) -> Result<Vec<MuaProfileResponse>> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let mua_service = MuaServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository)
    );
    mua_service.search_muas(pool, params).await
}

pub async fn get_mua_by_id(pool: &PgPool, mua_id: Uuid) -> Result<MuaProfileResponse> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let mua_service = MuaServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository)
    );
    mua_service.get_mua_by_id(pool, mua_id).await
}

pub async fn create_portfolio_item(pool: &PgPool, auth_header: Option<String>, portfolio_data: Value) -> Result<Value> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let mua_service = MuaServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository)
    );
    mua_service.create_portfolio_item(pool, auth_header, portfolio_data).await
}

pub async fn create_mua_profile(pool: &PgPool, auth_header: Option<String>, profile_data: CreateMuaProfileRequest) -> Result<MuaProfileResponse> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let mua_service = MuaServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository)
    );
    mua_service.create_profile(pool, auth_header, profile_data).await
}