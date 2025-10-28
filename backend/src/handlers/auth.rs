use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::models::{CreateUserRequest, LoginRequest};
use crate::services::auth_service;

pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    req: web::Json<CreateUserRequest>,
) -> impl Responder {
    match auth_service::register_user(&pool, req.into_inner()).await {
        Ok(auth_response) => HttpResponse::Created().json(auth_response),
        Err(e) => {
            let status = if e.to_string().contains("already exists") {
                actix_web::http::StatusCode::CONFLICT
            } else {
                actix_web::http::StatusCode::BAD_REQUEST
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

pub async fn login(
    pool: web::Data<sqlx::PgPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    match auth_service::login_user(&pool, req.into_inner()).await {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(_) => HttpResponse::Unauthorized().json(json!({
            "error": "Invalid credentials"
        }))
    }
}