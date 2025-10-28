use actix_web::{web, HttpResponse, Responder, HttpRequest};
use serde_json::json;
use crate::models::{SearchMuasRequest, MuaProfileResponse, CreateMuaProfileRequest};
use crate::services::mua_service;
use crate::services::user_service;
use crate::services::s3_service;

pub async fn get_muas(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<SearchMuasRequest>,
) -> impl Responder {
    match mua_service::search_muas(&pool, query.into_inner()).await {
        Ok(muas) => HttpResponse::Ok().json(muas),
        Err(e) => HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))
    }
}

pub async fn get_mua_by_id(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mua_id = path.into_inner();

    match mua_service::get_mua_by_id(&pool, mua_id).await {
        Ok(mua) => HttpResponse::Ok().json(mua),
        Err(_) => HttpResponse::NotFound().json(json!({
            "error": "MUA not found"
        }))
    }
}

pub async fn create_profile(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    profile_data: web::Json<CreateMuaProfileRequest>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match mua_service::create_mua_profile(&pool, auth_header, profile_data.into_inner()).await {
        Ok(profile) => HttpResponse::Created().json(profile),
        Err(e) => {
            let status = if e.to_string().contains("Unauthorized") {
                actix_web::http::StatusCode::UNAUTHORIZED
            } else if e.to_string().contains("already exists") {
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

pub async fn create_portfolio(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    portfolio_data: web::Json<serde_json::Value>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    println!("DEBUG: Creating portfolio with auth_header: {:?}", auth_header);
    println!("DEBUG: Portfolio data: {}", serde_json::to_string_pretty(&portfolio_data).unwrap_or_default());

    match mua_service::create_portfolio_item(&pool, auth_header, portfolio_data.into_inner()).await {
        Ok(portfolio) => {
            println!("DEBUG: Portfolio created successfully");
            HttpResponse::Created().json(portfolio)
        },
        Err(e) => {
            println!("DEBUG: Error creating portfolio: {}", e);
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
pub async fn get_mua_portfolio(
    pool: web::Data<sqlx::PgPool>,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {
    let mua_id = path.into_inner();

    match sqlx::query!(
        r#"
        SELECT id, title, description, image_url, service_type, created_at
        FROM portfolio_items
        WHERE mua_id = $1
        ORDER BY created_at DESC
        "#,
        mua_id
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(items) => {
            let portfolio_items: Vec<serde_json::Value> = items.into_iter().map(|item| {
                json!({
                    "id": item.id,
                    "title": item.title,
                    "description": item.description,
                    "image_url": item.image_url,
                    "service_type": item.service_type,
                    "created_at": item.created_at
                })
            }).collect();

            HttpResponse::Ok().json(portfolio_items)
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to fetch portfolio: {}", e)
            }))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GetPortfolioQuery {
    page: Option<i32>,
    limit: Option<i32>,
}

pub async fn get_current_mua_portfolio(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    query: web::Query<GetPortfolioQuery>,
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

    // Get pagination parameters
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).max(1).min(100);
    let offset = (page - 1) * limit;

    // First get MUA profile for this user
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

    // Get total count for pagination
    let total_count = match sqlx::query!(
        "SELECT COUNT(*) as count FROM portfolio_items WHERE mua_id = $1",
        mua_id
    )
    .fetch_one(&**pool)
    .await
    {
        Ok(count) => count.count.unwrap_or(0) as i64,
        Err(_) => 0,
    };

    // Get portfolio items with pagination
    match sqlx::query!(
        r#"
        SELECT id, title, description, image_url, service_type, created_at
        FROM portfolio_items
        WHERE mua_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        mua_id,
        limit as i32,
        offset as i32
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(items) => {
            let portfolio_items: Vec<serde_json::Value> = items.into_iter().map(|item| {
                json!({
                    "id": item.id,
                    "title": item.title,
                    "description": item.description,
                    "image_url": item.image_url,
                    "service_type": item.service_type,
                    "created_at": item.created_at
                })
            }).collect();

            let total_pages = (total_count as f64 / limit as f64).ceil() as i32;

            HttpResponse::Ok().json(json!({
                "data": portfolio_items,
                "pagination": {
                    "current_page": page,
                    "per_page": limit,
                    "total_items": total_count,
                    "total_pages": total_pages,
                    "has_next_page": page < total_pages,
                    "has_prev_page": page > 1
                }
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to fetch portfolio: {}", e)
            }))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct PresignedUrlRequest {
    file_name: String,
    content_type: String,
    folder: Option<String>,
}

#[derive(serde::Serialize)]
pub struct PresignedUrlResponse {
    presigned_url: String,
    public_url: String,
    expires_in: u64,
}

pub async fn get_presigned_upload_url(
    req: HttpRequest,
    body: web::Json<PresignedUrlRequest>,
) -> impl Responder {
    let auth_header = req.headers().get("authorization")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Validate authentication
    match user_service::get_user_id_from_auth_header(auth_header) {
        Ok(_) => {
            // User is authenticated, proceed with presigned URL generation
        }
        Err(e) => {
            return HttpResponse::Unauthorized().json(json!({
                "error": e.to_string()
            }));
        }
    }

    // Validate content type (only allow images)
    if !body.content_type.starts_with("image/") {
        return HttpResponse::BadRequest().json(json!({
            "error": "Only image files are allowed"
        }));
    }

    // Use provided folder or default to "uploads"
    let folder = body.folder.as_deref().unwrap_or("uploads");

    // Initialize S3 service
    let s3_service = match s3_service::S3Service::new().await {
        Ok(service) => service,
        Err(e) => {
            eprintln!("Failed to initialize S3 service: {}", e);
            return HttpResponse::InternalServerError().json(json!({
                "error": "Failed to initialize upload service"
            }));
        }
    };

    // Generate presigned URL
    match s3_service.get_presigned_upload_url(
        &body.file_name,
        &body.content_type,
        folder,
    ).await {
        Ok((presigned_url, public_url)) => {
            println!("DEBUG: Returning presigned URL: {}", presigned_url);
            println!("DEBUG: Returning public URL: {}", public_url);
            HttpResponse::Ok().json(PresignedUrlResponse {
                presigned_url,
                public_url,
                expires_in: 3600, // 1 hour in seconds
            })
        }
        Err(e) => {
            eprintln!("Failed to generate presigned URL: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": format!("Failed to generate upload URL: {}", e)
            }))
        }
    }
}

pub async fn debug_presigned_url() -> impl Responder {
    // Debug endpoint to test S3 service without authentication
    match s3_service::S3Service::new().await {
        Ok(service) => {
            match service.get_presigned_upload_url(
                "test.jpg",
                "image/jpeg",
                "portfolio"
            ).await {
                Ok((presigned_url, public_url)) => {
                    println!("DEBUG: Presigned URL: {}", presigned_url);
                    println!("DEBUG: Public URL: {}", public_url);
                    HttpResponse::Ok().json(serde_json::json!({
                        "presigned_url": presigned_url,
                        "public_url": public_url,
                        "message": "Debug URLs generated successfully"
                    }))
                }
                Err(e) => {
                    eprintln!("DEBUG: Failed to generate presigned URL: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": format!("Failed to generate presigned URL: {}", e)
                    }))
                }
            }
        }
        Err(e) => {
            eprintln!("DEBUG: Failed to initialize S3 service: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to initialize S3 service: {}", e)
            }))
        }
    }
}