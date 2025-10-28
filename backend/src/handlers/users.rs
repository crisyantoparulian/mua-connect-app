use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde_json::json;
use crate::models::UserResponse;
use crate::services::user_service;

pub async fn get_profile(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match user_service::get_user_profile(&pool, auth_header).await {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

pub async fn update_profile(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    profile_data: web::Json<serde_json::Value>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match user_service::update_user_profile(&pool, auth_header, profile_data.into_inner()).await {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(e) => {
            let status = match e.to_string().as_str() {
                "Unauthorized" => actix_web::http::StatusCode::UNAUTHORIZED,
                "Invalid input" => actix_web::http::StatusCode::BAD_REQUEST,
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}