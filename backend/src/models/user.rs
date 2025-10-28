use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub user_type: UserType,
    pub full_name: String,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_type", rename_all = "lowercase")]
pub enum UserType {
    Customer,
    Mua,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub user_type: UserType,
    pub full_name: String,
    pub phone_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub user_type: UserType,
    pub full_name: String,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            user_type: user.user_type,
            full_name: user.full_name,
            phone_number: user.phone_number,
            profile_picture_url: user.profile_picture_url,
            is_verified: user.is_verified,
            created_at: user.created_at,
        }
    }
}