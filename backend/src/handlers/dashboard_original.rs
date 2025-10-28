use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde_json::json;
use crate::models::{
    DashboardResponse, UpdateMuaAvailabilityRequest,
    PortfolioItem, CreatePortfolioRequest, UpdatePortfolioRequest
    // Temporarily commented out availability imports
    // CreateAvailabilityRequest, UpdateAvailabilityRequest as UpdateSlotRequest
};
use crate::services::dashboard_service;
use crate::services::user_service;

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

    match dashboard_service.update_availability(&pool, auth_header, availability_data.into_inner()).await {
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
        Ok(item) => HttpResponse::Created().json(item),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("MUA profile not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::BAD_REQUEST
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
    path: web::Path<uuid::Uuid>,
    portfolio_data: web::Json<UpdatePortfolioRequest>,
) -> impl Responder {
    let item_id = path.into_inner();
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

    match dashboard_service.update_portfolio_item(&pool, auth_header, item_id, portfolio_data.into_inner()).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::BAD_REQUEST
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
    path: web::Path<uuid::Uuid>,
) -> impl Responder {
    let item_id = path.into_inner();
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

    match dashboard_service.delete_portfolio_item(&pool, auth_header, item_id).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("not found") {
                actix_web::http::StatusCode::NOT_FOUND
            } else {
                actix_web::http::StatusCode::BAD_REQUEST
            };
            HttpResponse::build(status).json(json!({
                "error": e.to_string()
            }))
        }
    }
}

// Availability Management Handlers

#[derive(serde::Deserialize)]
pub struct GetAvailabilitySlotsQuery {
    month: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct GetCalendarBookingsQuery {
    start_date: String,
    end_date: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateBookingStatusRequest {
    status: String,
}

pub async fn get_availability_slots(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    query: web::Query<GetAvailabilitySlotsQuery>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Get user ID from JWT token
    let user_id = match user_service::get_user_id_from_auth_header(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": e.to_string()
            }));
        }
    };

    // Get MUA profile for this user
    let mua_id = match sqlx::query!(
        "SELECT id FROM mua_profiles WHERE user_id = $1",
        user_id
    )
    .fetch_one(&**pool)
    .await
    {
        Ok(mua) => mua.id,
        Err(_) => {
            return HttpResponse::NotFound().json(json!({
                "error": "MUA profile not found for this user"
            }));
        }
    };

    // Get availability slots for this MUA
    let mut slots_query = "SELECT id, mua_id, start_time, end_time, day_of_week, specific_date, is_available, recurring, created_at, updated_at FROM availability_slots WHERE mua_id = $1".to_string();

    if let Some(month) = &query.month {
        // Filter by month if provided
        slots_query.push_str(" AND (EXTRACT(MONTH FROM specific_date) = $2 OR specific_date IS NULL)");
    }

    slots_query.push_str(" ORDER BY created_at DESC");

    let result = if let Some(month) = &query.month {
        sqlx::query_as::<_, crate::models::AvailabilitySlot>(&slots_query)
            .bind(mua_id)
            .bind(month.parse::<i32>().unwrap_or(1))
            .fetch_all(&**pool)
            .await
    } else {
        sqlx::query_as::<_, crate::models::AvailabilitySlot>(&slots_query)
            .bind(mua_id)
            .fetch_all(&**pool)
            .await
    };

    match result {
        Ok(slots) => HttpResponse::Ok().json(slots),
        Err(e) => {
            eprintln!("DEBUG: Failed to fetch availability slots: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to fetch availability slots: {}", e)
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

    // Get user ID from JWT token
    let user_id = match user_service::get_user_id_from_auth_header(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": e.to_string()
            }));
        }
    };

    // Get MUA profile for this user
    let mua_id = match sqlx::query!(
        "SELECT id FROM mua_profiles WHERE user_id = $1",
        user_id
    )
    .fetch_one(&**pool)
    .await
    {
        Ok(mua) => mua.id,
        Err(_) => {
            return HttpResponse::NotFound().json(json!({
                "error": "MUA profile not found for this user"
            }));
        }
    };

    let data = availability_data.into_inner();

    // Handle multiple days for recurring slots
    if data.recurring && data.day_of_week.is_some() {
        let mut created_slots = Vec::new();

        for &day in data.day_of_week.as_ref().unwrap() {
            let slot_id = uuid::Uuid::new_v4();

            match sqlx::query!(
                r#"
                INSERT INTO availability_slots (id, mua_id, start_time, end_time, day_of_week, is_available, recurring, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
                RETURNING id, mua_id, start_time, end_time, day_of_week, specific_date, is_available, recurring, created_at, updated_at
                "#,
                slot_id,
                mua_id,
                data.start_time,
                data.end_time,
                day,
                true,
                data.recurring
            )
            .fetch_one(&**pool)
            .await
            {
                Ok(slot) => created_slots.push(slot),
                Err(e) => {
                    eprintln!("DEBUG: Failed to create availability slot: {}", e);
                    return HttpResponse::InternalServerError().json(json!({
                        "error": format!("Failed to create availability slot: {}", e)
                    }));
                }
            }
        }

        HttpResponse::Created().json(json!({
            "slots": created_slots,
            "message": format!("Created {} availability slots", created_slots.len())
        }))
    } else {
        // Single slot creation
        let slot_id = uuid::Uuid::new_v4();
        let specific_date = if !data.recurring {
            data.specific_date.map(|s| s.parse::<chrono::DateTime<chrono::Utc>>().unwrap_or_else(|_| chrono::Utc::now()))
        } else {
            None
        };

        match sqlx::query!(
            r#"
            INSERT INTO availability_slots (id, mua_id, start_time, end_time, day_of_week, specific_date, is_available, recurring, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
            RETURNING id, mua_id, start_time, end_time, day_of_week, specific_date, is_available, recurring, created_at, updated_at
            "#,
            slot_id,
            mua_id,
            data.start_time,
            data.end_time,
            data.day_of_week.map(|d| d[0]),
            specific_date,
            true,
            data.recurring
        )
        .fetch_one(&**pool)
        .await
        {
            Ok(slot) => HttpResponse::Created().json(slot),
            Err(e) => {
                eprintln!("DEBUG: Failed to create availability slot: {}", e);
                HttpResponse::InternalServerError().json(json!({
                    "error": format!("Failed to create availability slot: {}", e)
                }))
            }
        }
    }
}

pub async fn update_availability_slot(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    availability_data: web::Json<UpdateSlotRequest>,
) -> impl Responder {
    let slot_id = path.into_inner();
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Get user ID from JWT token
    let user_id = match user_service::get_user_id_from_auth_header(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": e.to_string()
            }));
        }
    };

    // Get MUA profile for this user
    let mua_id = match sqlx::query!(
        "SELECT id FROM mua_profiles WHERE user_id = $1",
        user_id
    )
    .fetch_one(&**pool)
    .await
    {
        Ok(mua) => mua.id,
        Err(_) => {
            return HttpResponse::NotFound().json(json!({
                "error": "MUA profile not found for this user"
            }));
        }
    };

    let data = availability_data.into_inner();

    // Build update query dynamically
    let mut updates = Vec::new();
    let mut values = Vec::new();
    let mut param_count = 1;

    updates.push(format!("is_available = ${}", param_count));
    values.push(data.is_available);
    param_count += 1;

    if updates.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "error": "No fields to update"
        }));
    }

    updates.push(format!("updated_at = NOW()"));
    values.push(slot_id);
    values.push(mua_id);

    let query = format!(
        "UPDATE availability_slots SET {} WHERE id = ${} AND mua_id = ${} RETURNING *",
        updates.join(", "),
        param_count,
        param_count + 1
    );

    match sqlx::query_as::<_, crate::models::AvailabilitySlot>(&query)
        .bind_all(values)
        .fetch_one(&**pool)
        .await
    {
        Ok(slot) => HttpResponse::Ok().json(slot),
        Err(e) => {
            eprintln!("DEBUG: Failed to update availability slot: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to update availability slot: {}", e)
            }))
        }
    }
}

pub async fn delete_availability_slot(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {
    let slot_id = path.into_inner();
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Get user ID from JWT token
    let user_id = match user_service::get_user_id_from_auth_header(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": e.to_string()
            }));
        }
    };

    // Get MUA profile for this user
    let mua_id = match sqlx::query!(
        "SELECT id FROM mua_profiles WHERE user_id = $1",
        user_id
    )
    .fetch_one(&**pool)
    .await
    {
        Ok(mua) => mua.id,
        Err(_) => {
            return HttpResponse::NotFound().json(json!({
                "error": "MUA profile not found for this user"
            }));
        }
    };

    match sqlx::query!(
        "DELETE FROM availability_slots WHERE id = $1 AND mua_id = $2",
        slot_id,
        mua_id
    )
    .execute(&**pool)
    .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({
            "message": "Availability slot deleted successfully"
        })),
        Err(e) => {
            eprintln!("DEBUG: Failed to delete availability slot: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to delete availability slot: {}", e)
            }))
        }
    }
}

pub async fn get_calendar_bookings(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    query: web::Query<GetCalendarBookingsQuery>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Get user ID from JWT token
    let user_id = match user_service::get_user_id_from_auth_header(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": e.to_string()
            }));
        }
    };

    // Get MUA profile for this user
    let mua_id = match sqlx::query!(
        "SELECT id FROM mua_profiles WHERE user_id = $1",
        user_id
    )
    .fetch_one(&**pool)
    .await
    {
        Ok(mua) => mua.id,
        Err(_) => {
            return HttpResponse::NotFound().json(json!({
                "error": "MUA profile not found for this user"
            }));
        }
    };

    // Get bookings for this MUA within the date range
    match sqlx::query!(
        r#"
        SELECT
            b.id,
            u.name as customer_name,
            u.phone as customer_phone,
            b.service_type,
            b.event_date,
            b.event_date + (b.duration_hours || ' hours')::INTERVAL as end_time,
            b.status,
            b.event_location,
            b.description as notes,
            b.price
        FROM bookings b
        JOIN users u ON b.customer_id = u.id
        WHERE b.mua_id = $1
        AND b.event_date >= $2
        AND b.event_date <= $3
        ORDER BY b.event_date ASC
        "#,
        mua_id,
        query.start_date,
        query.end_date
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(bookings) => {
            let calendar_bookings: Vec<crate::models::CalendarBooking> = bookings.into_iter().map(|booking| {
                crate::models::CalendarBooking {
                    id: booking.id,
                    customer_name: booking.customer_name,
                    customer_phone: booking.customer_phone,
                    service_type: booking.service_type,
                    start_time: booking.event_date,
                    end_time: booking.end_time,
                    status: booking.status.to_lowercase(),
                    location: Some(booking.event_location),
                    notes: booking.notes,
                    price: booking.price,
                }
            }).collect();

            HttpResponse::Ok().json(calendar_bookings)
        },
        Err(e) => {
            eprintln!("DEBUG: Failed to fetch calendar bookings: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to fetch calendar bookings: {}", e)
            }))
        }
    }
}

pub async fn update_booking_status_calendar(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    path: web::Path<uuid::Uuid>,
    status_data: web::Json<UpdateBookingStatusRequest>,
) -> impl Responder {
    let booking_id = path.into_inner();
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Get user ID from JWT token
    let user_id = match user_service::get_user_id_from_auth_header(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": e.to_string()
            }));
        }
    };

    // Get MUA profile for this user
    let mua_id = match sqlx::query!(
        "SELECT id FROM mua_profiles WHERE user_id = $1",
        user_id
    )
    .fetch_one(&**pool)
    .await
    {
        Ok(mua) => mua.id,
        Err(_) => {
            return HttpResponse::NotFound().json(json!({
                "error": "MUA profile not found for this user"
            }));
        }
    };

    // Update booking status
    match sqlx::query!(
        "UPDATE bookings SET status = $1, updated_at = NOW() WHERE id = $2 AND mua_id = $3 RETURNING *",
        status_data.status,
        booking_id,
        mua_id
    )
    .fetch_one(&**pool)
    .await
    {
        Ok(booking) => {
            let calendar_booking = crate::models::CalendarBooking {
                id: booking.id,
                customer_name: "".to_string(), // Would need to join with users table
                customer_phone: None,
                service_type: booking.service_type,
                start_time: booking.event_date,
                end_time: booking.event_date + chrono::Duration::hours(booking.duration_hours as i64),
                status: booking.status.to_lowercase(),
                location: Some(booking.event_location),
                notes: booking.description,
                price: booking.price,
            };
            HttpResponse::Ok().json(calendar_booking)
        },
        Err(e) => {
            eprintln!("DEBUG: Failed to update booking status: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to update booking status: {}", e)
            }))
        }
    }
}