use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde_json::json;
use crate::models::{CreateBookingRequest, UpdateBookingStatusRequest};
use crate::services::booking_service;

pub async fn create_booking(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    booking_data: web::Json<CreateBookingRequest>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match booking_service::create_booking(&pool, auth_header, booking_data.into_inner()).await {
        Ok(booking) => HttpResponse::Created().json(booking),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else {
                actix_web::http::StatusCode::BAD_REQUEST
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

pub async fn get_bookings(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match booking_service::get_user_bookings(&pool, auth_header).await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
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

pub async fn update_booking_status(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    status_data: web::Json<UpdateBookingStatusRequest>,
) -> impl Responder {
    let booking_id = path.into_inner();
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match booking_service::update_booking_status(&pool, auth_header, booking_id, status_data.into_inner()).await {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(e) => {
            let status = match e.to_string().as_str() {
                "Unauthorized" => actix_web::http::StatusCode::UNAUTHORIZED,
                "Booking not found" => actix_web::http::StatusCode::NOT_FOUND,
                _ => actix_web::http::StatusCode::BAD_REQUEST,
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}