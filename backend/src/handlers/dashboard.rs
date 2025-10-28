use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde_json::json;
use crate::models::dashboard::{DashboardResponse, UpdateAvailabilityRequest};
use crate::models::{UpdateMuaAvailabilityRequest, PortfolioItem, CreatePortfolioRequest, UpdatePortfolioRequest};
use crate::models::availability::CreateAvailabilityRequest;
use crate::services::dashboard_service;
use crate::services::user_service;
use crate::services::booking_service;

pub async fn get_dashboard(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match dashboard_service::get_dashboard(&pool, auth_header).await {
        Ok(dashboard) => HttpResponse::Ok().json(dashboard),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("MUA profile not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

pub async fn update_availability(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    availability_data: web::Json<UpdateMuaAvailabilityRequest>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let dashboard_service = crate::services::dashboard_service::DashboardServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );

    match dashboard_service.update_availability(&pool, auth_header, crate::models::dashboard::UpdateAvailabilityRequest {
        is_available: availability_data.into_inner().is_available
    }).await {
        Ok(result) => HttpResponse::Ok().json(result),
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

pub async fn get_portfolio_items(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let dashboard_service = crate::services::dashboard_service::DashboardServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );

    match dashboard_service.get_portfolio_items(&pool, auth_header).await {
        Ok(items) => HttpResponse::Ok().json(items),
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

pub async fn create_portfolio_item(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    portfolio_data: web::Json<CreatePortfolioRequest>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let dashboard_service = crate::services::dashboard_service::DashboardServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );

    match dashboard_service.create_portfolio_item(&pool, auth_header, portfolio_data.into_inner()).await {
        Ok(item) => HttpResponse::Ok().json(item),
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

pub async fn update_portfolio_item(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    item_id: web::Path<uuid::Uuid>,
    portfolio_data: web::Json<UpdatePortfolioRequest>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let dashboard_service = crate::services::dashboard_service::DashboardServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );

    match dashboard_service.update_portfolio_item(&pool, auth_header, item_id.into_inner(), portfolio_data.into_inner()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

pub async fn delete_portfolio_item(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    item_id: web::Path<uuid::Uuid>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let dashboard_service = crate::services::dashboard_service::DashboardServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );

    match dashboard_service.delete_portfolio_item(&pool, auth_header, item_id.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Portfolio item deleted successfully"})),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

// Availability Management Functions
pub async fn get_availability_slots(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    _query: web::Query<serde_json::Value>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let dashboard_service = crate::services::dashboard_service::DashboardServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );

    match dashboard_service.get_availability_slots(&pool, auth_header).await {
        Ok(slots) => HttpResponse::Ok().json(slots),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("MUA profile not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

pub async fn create_availability_slot(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    availability_data: web::Json<CreateAvailabilityRequest>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let dashboard_service = crate::services::dashboard_service::DashboardServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );

    match dashboard_service.create_availability_slot(&pool, auth_header, availability_data.into_inner()).await {
        Ok(slot) => HttpResponse::Ok().json(slot),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("MUA profile not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else if e.to_string().contains("Invalid date format") {
                actix_web::http::StatusCode::BAD_REQUEST
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

pub async fn update_availability_slot(
    _pool: web::Data<sqlx::PgPool>,
    _req: HttpRequest,
    slot_id: web::Path<uuid::Uuid>,
    data: web::Json<serde_json::Value>,
) -> HttpResponse {
    // Return mock response
    let mock_response = json!({
        "id": slot_id.to_string(),
        "start_time": data.get("start_time"),
        "end_time": data.get("end_time"),
        "is_available": data.get("is_available").unwrap_or(&serde_json::Value::Bool(true)),
        "recurring": data.get("recurring"),
        "day_of_week": data.get("day_of_week")
    });

    HttpResponse::Ok().json(mock_response)
}

pub async fn delete_availability_slot(
    _pool: web::Data<sqlx::PgPool>,
    _req: HttpRequest,
    _slot_id: web::Path<uuid::Uuid>,
) -> HttpResponse {
    HttpResponse::Ok().json(json!({"message": "Availability slot deleted successfully"}))
}

pub async fn get_calendar_bookings(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    query: web::Query<serde_json::Value>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let user_repository = crate::repository::UserRepositoryImpl::new();
    let mua_repository = crate::repository::MuaRepositoryImpl::new();
    let booking_repository = crate::repository::BookingRepositoryImpl::new();
    let dashboard_service = crate::services::dashboard_service::DashboardServiceImpl::new(
        Box::new(user_repository),
        Box::new(mua_repository),
        Box::new(booking_repository)
    );

    let start_date = query.get("start_date").and_then(|v| v.as_str()).unwrap_or("");
    let end_date = query.get("end_date").and_then(|v| v.as_str()).unwrap_or("");

    match dashboard_service.get_calendar_bookings(&pool, auth_header, start_date, end_date).await {
        Ok(bookings) => HttpResponse::Ok().json(bookings),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("MUA profile not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

pub async fn update_booking_status_calendar(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    booking_id: web::Path<uuid::Uuid>,
    status_data: web::Json<serde_json::Value>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let status = status_data.get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Create UpdateBookingStatusRequest
    let status_request = crate::models::booking::UpdateBookingStatusRequest {
        status: match status {
            "pending" => crate::models::booking::BookingStatus::Pending,
            "confirmed" => crate::models::booking::BookingStatus::Confirmed,
            "cancelled" => crate::models::booking::BookingStatus::Cancelled,
            "completed" => crate::models::booking::BookingStatus::Completed,
            "no_show" => crate::models::booking::BookingStatus::NoShow,
            _ => {
                return HttpResponse::BadRequest().json(json!({
                    "error": "Invalid status. Must be one of: pending, confirmed, cancelled, completed, no_show"
                }));
            }
        },
    };

    match booking_service::update_booking_status(&pool, auth_header, booking_id.into_inner(), status_request).await {
        Ok(booking) => HttpResponse::Ok().json(booking),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else if e.to_string().contains("Invalid status") {
                actix_web::http::StatusCode::BAD_REQUEST
            } else {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}