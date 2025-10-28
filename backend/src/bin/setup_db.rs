use sqlx::PgPool;
use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    println!("Creating database tables...");

    // Create types
    sqlx::query("CREATE TYPE user_type AS ENUM ('customer', 'mua');")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE TYPE booking_status AS ENUM ('pending', 'confirmed', 'cancelled', 'completed', 'no_show');")
        .execute(&pool)
        .await?;

    // Create users table
    sqlx::query(
        r#"
        CREATE TABLE users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            user_type user_type NOT NULL DEFAULT 'customer',
            full_name VARCHAR(255) NOT NULL,
            phone_number VARCHAR(20),
            profile_picture_url TEXT,
            is_verified BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // Create mua_profiles table
    sqlx::query(
        r#"
        CREATE TABLE mua_profiles (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            bio TEXT,
            experience_years INTEGER,
            specialization TEXT[],
            location VARCHAR(255) NOT NULL,
            latitude DECIMAL(10, 8),
            longitude DECIMAL(11, 8),
            is_available BOOLEAN DEFAULT TRUE,
            average_rating DECIMAL(3, 2),
            total_reviews INTEGER DEFAULT 0,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // Create portfolio_items table
    sqlx::query(
        r#"
        CREATE TABLE portfolio_items (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            mua_id UUID NOT NULL REFERENCES mua_profiles(id) ON DELETE CASCADE,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            image_url TEXT NOT NULL,
            service_type VARCHAR(100),
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // Create bookings table
    sqlx::query(
        r#"
        CREATE TABLE bookings (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            customer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            mua_id UUID NOT NULL REFERENCES mua_profiles(id) ON DELETE CASCADE,
            service_type VARCHAR(255) NOT NULL,
            description TEXT,
            event_date TIMESTAMP WITH TIME ZONE NOT NULL,
            event_location VARCHAR(255) NOT NULL,
            duration_hours INTEGER NOT NULL,
            price DECIMAL(10, 2) NOT NULL,
            status booking_status DEFAULT 'pending',
            deposit_amount DECIMAL(10, 2),
            deposit_paid BOOLEAN DEFAULT FALSE,
            final_payment_paid BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // Create reviews table
    sqlx::query(
        r#"
        CREATE TABLE reviews (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            booking_id UUID NOT NULL REFERENCES bookings(id) ON DELETE CASCADE,
            reviewer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            reviewee_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
            comment TEXT,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // Create availability table
    sqlx::query(
        r#"
        CREATE TABLE availability (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            mua_id UUID NOT NULL REFERENCES mua_profiles(id) ON DELETE CASCADE,
            date DATE NOT NULL,
            start_time TIME NOT NULL,
            end_time TIME NOT NULL,
            is_available BOOLEAN DEFAULT TRUE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // Create messages table
    sqlx::query(
        r#"
        CREATE TABLE messages (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            booking_id UUID REFERENCES bookings(id) ON DELETE CASCADE,
            sender_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            receiver_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            content TEXT NOT NULL,
            is_read BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await?;

    println!("Database tables created successfully!");
    Ok(())
}