use anyhow::Result;
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Duration, Utc};
use uuid::Uuid;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

use crate::models::{CreateUserRequest, LoginRequest, AuthResponse, User, UserResponse, UserType, CreateMuaProfileRequest};
use crate::repository::traits::{UserRepository, MuaRepository};
use super::traits::AuthService;

#[derive(Debug, Serialize, Deserialize, Default)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct AuthServiceImpl {
    user_repository: Box<dyn UserRepository>,
    mua_repository: Box<dyn MuaRepository>,
}

impl AuthServiceImpl {
    pub fn new(user_repository: Box<dyn UserRepository>, mua_repository: Box<dyn MuaRepository>) -> Self {
        Self { user_repository, mua_repository }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn register(&self, pool: &PgPool, req: CreateUserRequest) -> Result<AuthResponse> {
        let existing_user = self.user_repository.find_by_email(pool, &req.email).await?;

        if existing_user.is_some() {
            return Err(anyhow::anyhow!("User with this email already exists"));
        }

        let password_hash = hash(&req.password, DEFAULT_COST)?;
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        let user = User {
            id: user_id,
            email: req.email.clone(),
            password_hash,
            user_type: req.user_type,
            full_name: req.full_name,
            phone_number: req.phone_number,
            profile_picture_url: None,
            is_verified: false,
            created_at: now,
            updated_at: now,
        };

        let created_user = self.user_repository.create_user(pool, &user).await?;

        // If user is registering as MUA, automatically create an MUA profile
        if user.user_type == crate::models::UserType::Mua {
            let mua_profile_request = CreateMuaProfileRequest {
                bio: None,
                experience_years: None,
                specialization: None,
                location: "".to_string(), // Will need to be updated later
                latitude: None,
                longitude: None,
                profile_picture_base64: None,
                profile_picture_url: None,
            };

            // Create MUA profile with default empty location
            self.mua_repository.create_mua_profile(pool, user_id, mua_profile_request).await?;
        }

        let token = generate_jwt_token(user_id)?;

        Ok(AuthResponse {
            user: UserResponse::from(created_user),
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 604800, // 7 days
        })
    }

    async fn login(&self, pool: &PgPool, req: LoginRequest) -> Result<AuthResponse> {
        let user = self.user_repository.find_by_email(pool, &req.email).await?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        let is_valid = verify(&req.password, &user.password_hash)?;
        if !is_valid {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let token = generate_jwt_token(user.id)?;

        Ok(AuthResponse {
            user: UserResponse::from(user),
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 604800, // 7 days
        })
    }

    async fn verify_token(&self, token: &str) -> Result<Uuid> {
        verify_jwt_token(token)
    }
}

pub fn generate_jwt_token(user_id: Uuid) -> Result<String> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-super-secret-jwt-key".to_string());

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("Valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
        ..Default::default()
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| anyhow::anyhow!("Failed to generate token: {}", e))
}

pub fn verify_jwt_token(token: &str) -> Result<Uuid> {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-super-secret-jwt-key".to_string());

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    let user_id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|e| anyhow::anyhow!("Invalid user ID in token: {}", e))?;

    Ok(user_id)
}

// Legacy functions for backward compatibility
pub async fn register_user(pool: &PgPool, req: crate::models::CreateUserRequest) -> Result<AuthResponse> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let auth_service = AuthServiceImpl::new(Box::new(user_repository), Box::new(mua_repository));
    auth_service.register(pool, req).await
}

pub async fn login_user(pool: &PgPool, req: crate::models::LoginRequest) -> Result<AuthResponse> {
    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let auth_service = AuthServiceImpl::new(Box::new(user_repository), Box::new(mua_repository));
    auth_service.login(pool, req).await
}