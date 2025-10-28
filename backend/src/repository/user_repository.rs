use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::{PgPool, query_as, query, Row};
use serde_json::Value;

use crate::models::{User, UserType};
use super::traits::UserRepository;

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl;

impl UserRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UserRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, pool: &PgPool, email: &str) -> Result<Option<User>> {
        let user = query_as::<_, User>(
            "SELECT id, email, password_hash, user_type, full_name, phone_number, profile_picture_url, is_verified, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    async fn find_by_id(&self, pool: &PgPool, id: Uuid) -> Result<Option<User>> {
        let user = query_as::<_, User>(
            "SELECT id, email, password_hash, user_type, full_name, phone_number, profile_picture_url, is_verified, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    async fn create_user(&self, pool: &PgPool, user: &User) -> Result<User> {
        let created_user = query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, user_type, full_name, phone_number, profile_picture_url, is_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, email, password_hash, user_type, full_name, phone_number, profile_picture_url, is_verified, created_at, updated_at
            "#
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.user_type as UserType)
        .bind(&user.full_name)
        .bind(&user.phone_number)
        .bind(&user.profile_picture_url)
        .bind(user.is_verified)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(pool)
        .await?;

        Ok(created_user)
    }

    async fn update_user(&self, pool: &PgPool, id: Uuid, updates: &Value) -> Result<User> {
        let mut query = String::from("UPDATE users SET updated_at = NOW()");
        let mut params: Vec<String> = Vec::new();
        let mut param_index = 1;

        if let Some(full_name) = updates.get("full_name").and_then(|v| v.as_str()) {
            query.push_str(&format!(", full_name = ${}", param_index));
            params.push(full_name.to_string());
            param_index += 1;
        }

        if let Some(phone_number) = updates.get("phone_number").and_then(|v| v.as_str()) {
            query.push_str(&format!(", phone_number = ${}", param_index));
            params.push(phone_number.to_string());
            param_index += 1;
        }

        if let Some(profile_picture_url) = updates.get("profile_picture_url").and_then(|v| v.as_str()) {
            query.push_str(&format!(", profile_picture_url = ${}", param_index));
            params.push(profile_picture_url.to_string());
            param_index += 1;
        }

        query.push_str(&format!(" WHERE id = ${} RETURNING *", param_index));
        params.push(id.to_string());

        let mut query_builder = query_as::<_, User>(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        let updated_user = query_builder
            .fetch_one(pool)
            .await?;

        Ok(updated_user)
    }

    async fn get_user_type(&self, pool: &PgPool, id: Uuid) -> Result<String> {
        let row = query("SELECT user_type FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        let user_type: String = row.get("user_type");
        Ok(user_type)
    }
}