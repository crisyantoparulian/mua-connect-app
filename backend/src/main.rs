use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

mod handlers;
mod models;
mod services;
mod repository;
mod utils;

async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Check if table already exists
    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'availability_slots')"
    )
    .fetch_one(pool)
    .await?;

    if table_exists {
        println!("✅ Availability slots table already exists");
        return Ok(());
    }

    println!("📝 Creating availability_slots table...");

    // Create table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS availability_slots (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            mua_id UUID NOT NULL REFERENCES mua_profiles(id) ON DELETE CASCADE,
            start_time TIME NOT NULL,
            end_time TIME NOT NULL,
            day_of_week INTEGER CHECK (day_of_week >= 0 AND day_of_week <= 6),
            specific_date TIMESTAMP WITH TIME ZONE,
            is_available BOOLEAN NOT NULL DEFAULT true,
            recurring BOOLEAN NOT NULL DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#
    )
    .execute(pool)
    .await?;

    // Add constraints separately
    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_constraint
                WHERE conname = 'check_recurring_logic'
                AND conrelid = 'availability_slots'::regclass
            ) THEN
                ALTER TABLE availability_slots
                ADD CONSTRAINT check_recurring_logic CHECK (
                    (recurring = true AND day_of_week IS NOT NULL AND specific_date IS NULL) OR
                    (recurring = false AND day_of_week IS NULL AND specific_date IS NOT NULL)
                );
            END IF;
        END $$;
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_constraint
                WHERE conname = 'check_time_order'
                AND conrelid = 'availability_slots'::regclass
            ) THEN
                ALTER TABLE availability_slots
                ADD CONSTRAINT check_time_order CHECK (end_time > start_time);
            END IF;
        END $$;
        "#
    )
    .execute(pool)
    .await?;

    // Create indexes safely
    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_indexes
                WHERE tablename = 'availability_slots'
                AND indexname = 'idx_availability_slots_mua_id'
            ) THEN
                CREATE INDEX idx_availability_slots_mua_id ON availability_slots(mua_id);
            END IF;
        END $$;
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_indexes
                WHERE tablename = 'availability_slots'
                AND indexname = 'idx_availability_slots_day_of_week'
            ) THEN
                CREATE INDEX idx_availability_slots_day_of_week ON availability_slots(day_of_week) WHERE day_of_week IS NOT NULL;
            END IF;
        END $$;
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_indexes
                WHERE tablename = 'availability_slots'
                AND indexname = 'idx_availability_slots_specific_date'
            ) THEN
                CREATE INDEX idx_availability_slots_specific_date ON availability_slots(specific_date) WHERE specific_date IS NOT NULL;
            END IF;
        END $$;
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_indexes
                WHERE tablename = 'availability_slots'
                AND indexname = 'idx_availability_slots_is_available'
            ) THEN
                CREATE INDEX idx_availability_slots_is_available ON availability_slots(is_available);
            END IF;
        END $$;
        "#
    )
    .execute(pool)
    .await?;

    // Create trigger function
    sqlx::query(
        r#"
        CREATE OR REPLACE FUNCTION update_updated_at_column()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = NOW();
            RETURN NEW;
        END;
        $$ language 'plpgsql';
        "#
    )
    .execute(pool)
    .await?;

    // Create trigger safely
    sqlx::query("DROP TRIGGER IF EXISTS update_availability_slots_updated_at ON availability_slots")
        .execute(pool).await?;

    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_trigger
                WHERE tgname = 'update_availability_slots_updated_at'
            ) THEN
                CREATE TRIGGER update_availability_slots_updated_at
                    BEFORE UPDATE ON availability_slots
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
            END IF;
        END $$;
        "#
    )
    .execute(pool)
    .await?;

    println!("✅ Availability slots migration completed successfully");
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Run migrations
    run_migrations(&pool).await.expect("Failed to run migrations");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://localhost:5174")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["authorization", "content-type"])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(handlers::auth::register))
                            .route("/login", web::post().to(handlers::auth::login))
                    )
                    .service(
                        web::scope("/users")
                            .route("/profile", web::get().to(handlers::users::get_profile))
                            .route("/profile", web::put().to(handlers::users::update_profile))
                    )
                    .service(
                        web::scope("/muas")
                            .route("/search", web::get().to(handlers::muas::get_muas))
                            .route("/profile", web::post().to(handlers::muas::create_profile))
                            .service(
                                web::resource("/portfolio")
                                    .route(web::get().to(handlers::muas::get_current_mua_portfolio))
                                    .route(web::post().to(handlers::muas::create_portfolio))
                            )
                            .route("/upload/presigned", web::post().to(handlers::muas::get_presigned_upload_url))
                            .route("/debug/presigned", web::get().to(handlers::muas::debug_presigned_url))
                            .route("/{id}", web::get().to(handlers::muas::get_mua_by_id))
                            .route("/{id}/portfolio", web::get().to(handlers::muas::get_mua_portfolio))
                    )
                    .service(
                        web::scope("/bookings")
                            .route("", web::post().to(handlers::bookings::create_booking))
                            .route("", web::get().to(handlers::bookings::get_bookings))
                            .route("/{id}/status", web::put().to(handlers::bookings::update_booking_status))
                    )
                    .service(
                        web::scope("/dashboard")
                            .route("", web::get().to(handlers::dashboard::get_dashboard))
                            .route("/availability", web::put().to(handlers::dashboard::update_availability))
                            // Availability Management endpoints
                            .route("/availability/slots", web::get().to(handlers::dashboard::get_availability_slots))
                            .route("/availability/slots", web::post().to(handlers::dashboard::create_availability_slot))
                            .route("/availability/slots/{id}", web::put().to(handlers::dashboard::update_availability_slot))
                            .route("/availability/slots/{id}", web::delete().to(handlers::dashboard::delete_availability_slot))
                            .route("/calendar/bookings", web::get().to(handlers::dashboard::get_calendar_bookings))
                            .route("/bookings/{id}/status", web::put().to(handlers::dashboard::update_booking_status_calendar))
                    )
                    .service(
                        web::scope("/portfolio")
                            .route("", web::get().to(handlers::dashboard::get_portfolio_items))
                            .route("", web::post().to(handlers::dashboard::create_portfolio_item))
                            .route("/{id}", web::put().to(handlers::dashboard::update_portfolio_item))
                            .route("/{id}", web::delete().to(handlers::dashboard::delete_portfolio_item))
                    )
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
