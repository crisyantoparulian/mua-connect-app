use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;
use sqlx::{PgPool, query, Row};
use serde_json::Value;

use crate::models::{MuaProfileResponse, CreateMuaProfileRequest, SearchMuasRequest, User};
use super::traits::MuaRepository;

#[derive(Debug, Clone)]
pub struct MuaRepositoryImpl;

impl MuaRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MuaRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MuaRepository for MuaRepositoryImpl {
    async fn search_muas(&self, pool: &PgPool, params: &SearchMuasRequest) -> Result<Vec<MuaProfileResponse>> {
        let mut sql = String::from(
            "SELECT mp.id as mua_id, mp.user_id, mp.bio, mp.experience_years, mp.specialization,
                    mp.location, mp.latitude, mp.longitude, mp.is_available, mp.average_rating,
                    mp.total_reviews, mp.created_at as mua_created_at, mp.updated_at as mua_updated_at,
                    u.id as user_id, u.email, u.password_hash, u.user_type, u.full_name,
                    u.phone_number, u.profile_picture_url, u.is_verified, u.created_at, u.updated_at
             FROM mua_profiles mp
             JOIN users u ON mp.user_id = u.id
             WHERE u.user_type = 'mua'"
        );

        let mut conditions = Vec::new();

        if let Some(location) = &params.location {
            if !location.is_empty() {
                conditions.push(format!("mp.location ILIKE '%{}%'", location));
            }
        }

        if let Some(min_rating) = params.min_rating {
            if min_rating > 0.0 {
                conditions.push(format!("mp.average_rating >= {}", min_rating));
            }
        }

        if let Some(specialization) = &params.specialization {
            if !specialization.is_empty() {
                conditions.push(format!("mp.specialization @> ARRAY['{}']", specialization));
            }
        }

        if !conditions.is_empty() {
            sql.push_str(" AND ");
            sql.push_str(&conditions.join(" AND "));
        }

        sql.push_str(" ORDER BY mp.average_rating DESC NULLS LAST");

        if let Some(limit) = params.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(page) = params.page {
            let offset = (page - 1) * params.limit.unwrap_or(12);
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        let rows = query(&sql)
            .fetch_all(pool)
            .await?;

        let results = rows.into_iter().map(|row| -> anyhow::Result<MuaProfileResponse> {
            Ok(MuaProfileResponse {
                id: row.get("mua_id"),
                user: User {
                    id: row.get("user_id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    user_type: row.get("user_type"),
                    full_name: row.get("full_name"),
                    phone_number: row.get("phone_number"),
                    profile_picture_url: row.get("profile_picture_url"),
                    is_verified: row.get("is_verified"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                },
                bio: row.get("bio"),
                experience_years: row.get("experience_years"),
                specialization: row.get("specialization"),
                location: row.get("location"),
                latitude: row.get::<Option<sqlx::types::BigDecimal>, _>("latitude")
                    .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0)),
                longitude: row.get::<Option<sqlx::types::BigDecimal>, _>("longitude")
                    .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0)),
                is_available: row.get("is_available"),
                average_rating: row.get("average_rating"),
                total_reviews: row.get("total_reviews"),
                created_at: row.get("mua_created_at"),
            })
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(results)
    }

    async fn get_mua_by_id(&self, pool: &PgPool, mua_id: Uuid) -> Result<Option<MuaProfileResponse>> {
        let row = query(
            "SELECT mp.*, u.* FROM mua_profiles mp
             JOIN users u ON mp.user_id = u.id
             WHERE mp.id = $1"
        )
        .bind(mua_id)
        .fetch_optional(pool)
        .await?;

        match row {
            Some(row) => Ok(Some(MuaProfileResponse {
                id: row.get("id"),
                user: User {
                    id: row.get("user_id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    user_type: row.get("user_type"),
                    full_name: row.get("full_name"),
                    phone_number: row.get("phone_number"),
                    profile_picture_url: row.get("profile_picture_url"),
                    is_verified: row.get("is_verified"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                },
                bio: row.get("bio"),
                experience_years: row.get("experience_years"),
                specialization: row.get("specialization"),
                location: row.get("location"),
                latitude: row.get::<Option<sqlx::types::BigDecimal>, _>("latitude")
                    .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0)),
                longitude: row.get::<Option<sqlx::types::BigDecimal>, _>("longitude")
                    .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0)),
                is_available: row.get("is_available"),
                average_rating: row.get("average_rating"),
                total_reviews: row.get("total_reviews"),
                created_at: row.get("created_at"),
            })),
            None => Ok(None),
        }
    }

    async fn get_mua_by_user_id(&self, pool: &PgPool, user_id: Uuid) -> Result<Option<Uuid>> {
        let row = query("SELECT id FROM mua_profiles WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?;

        Ok(row.map(|r| r.get("id")))
    }

    async fn create_mua_profile(&self, pool: &PgPool, user_id: Uuid, profile_data: CreateMuaProfileRequest) -> Result<MuaProfileResponse> {
        // First get the user to include in the response
        let user_row = query("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(pool)
            .await?;

        let user = User {
            id: user_row.get("id"),
            email: user_row.get("email"),
            password_hash: user_row.get("password_hash"),
            user_type: user_row.get("user_type"),
            full_name: user_row.get("full_name"),
            phone_number: user_row.get("phone_number"),
            profile_picture_url: user_row.get("profile_picture_url"),
            is_verified: user_row.get("is_verified"),
            created_at: user_row.get("created_at"),
            updated_at: user_row.get("updated_at"),
        };

        // Insert the MUA profile
        let row = query(
            r#"
            INSERT INTO mua_profiles (user_id, bio, experience_years, specialization, location, latitude, longitude, is_available, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, true, NOW(), NOW())
            RETURNING id, bio, experience_years, specialization, location, latitude, longitude, is_available, average_rating, total_reviews, created_at, updated_at
            "#
        )
        .bind(user_id)
        .bind(&profile_data.bio)
        .bind(profile_data.experience_years)
        .bind(&profile_data.specialization)
        .bind(&profile_data.location)
        .bind(profile_data.latitude)
        .bind(profile_data.longitude)
        .fetch_one(pool)
        .await?;

        Ok(MuaProfileResponse {
            id: row.get("id"),
            user,
            bio: row.get("bio"),
            experience_years: row.get("experience_years"),
            specialization: row.get("specialization"),
            location: row.get("location"),
            latitude: row.get::<Option<sqlx::types::BigDecimal>, _>("latitude")
                .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0)),
            longitude: row.get::<Option<sqlx::types::BigDecimal>, _>("longitude")
                .map(|bd| bd.to_string().parse::<f64>().unwrap_or(0.0)),
            is_available: row.get("is_available"),
            average_rating: row.get("average_rating"),
            total_reviews: row.get("total_reviews"),
            created_at: row.get("created_at"),
        })
    }

    async fn create_portfolio_item(&self, pool: &PgPool, mua_id: Uuid, portfolio_data: &Value) -> Result<Value> {
        let title = portfolio_data.get("title")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Title is required"))?;

        let image_url = portfolio_data.get("image_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Image URL is required"))?;

        let portfolio_item = query(
            r#"
            INSERT INTO portfolio_items (mua_id, title, description, image_url, service_type, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING id, title, description, image_url, service_type, created_at
            "#
        )
        .bind(mua_id)
        .bind(title)
        .bind(portfolio_data.get("description").and_then(|v| v.as_str()))
        .bind(image_url)
        .bind(portfolio_data.get("service_type").and_then(|v| v.as_str()))
        .fetch_one(pool)
        .await?;

        let response = serde_json::json!({
            "id": portfolio_item.get::<Uuid, _>("id"),
            "title": portfolio_item.get::<String, _>("title"),
            "description": portfolio_item.get::<Option<String>, _>("description"),
            "image_url": portfolio_item.get::<String, _>("image_url"),
            "service_type": portfolio_item.get::<Option<String>, _>("service_type"),
            "created_at": portfolio_item.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
        });

        Ok(response)
    }
}