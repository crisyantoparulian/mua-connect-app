-- Create some test customer bookings for demonstration
-- This will show how bookings appear on the MUA calendar

-- First, let's create a test customer user
INSERT INTO users (id, email, password_hash, user_type, full_name, phone_number, is_verified, created_at, updated_at)
VALUES (
    '550e8400-e29b-41d4-a716-446655440001',
    'test.customer@example.com',
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.s5uO8G', -- password: password123
    'customer',
    'Sarah Johnson',
    '+1234567890',
    true,
    NOW(),
    NOW()
) ON CONFLICT (id) DO NOTHING;

-- Create some test bookings for the current MUA
-- Replace 'YOUR_MUA_ID' with the actual MUA ID from your system

INSERT INTO bookings (
    id,
    customer_id,
    mua_id,
    service_type,
    description,
    event_date,
    event_location,
    duration_hours,
    price,
    status,
    created_at,
    updated_at
) VALUES
-- Booking 1: Today - Bridal Makeup
(
    '550e8400-e29b-41d4-a716-446655440002',
    '550e8400-e29b-41d4-a716-446655440001',
    (SELECT id FROM mua_profiles LIMIT 1), -- Get first MUA profile
    'Bridal Makeup',
    'Bridal makeup for wedding ceremony',
    '2025-10-15 14:00:00+00',
    'Grand Ballroom, Jakarta',
    4,
    2500000.00,
    'confirmed',
    NOW(),
    NOW()
),
-- Booking 2: Tomorrow - Party Makeup
(
    '550e8400-e29b-41d4-a716-446655440003',
    '550e8400-e29b-41d4-a716-446655440001',
    (SELECT id FROM mua_profiles LIMIT 1), -- Get first MUA profile
    'Party Makeup',
    'Birthday party makeup',
    '2025-10-16 18:00:00+00',
    'Menteng, Jakarta',
    2,
    800000.00,
    'pending',
    NOW(),
    NOW()
),
-- Booking 3: This Weekend - Photoshoot
(
    '550e8400-e29b-41d4-a716-446655440004',
    '550e8400-e29b-41d4-a716-446655440001',
    (SELECT id FROM mua_profiles LIMIT 1), -- Get first MUA profile
    'Photoshoot Makeup',
    'Professional photoshoot for portfolio',
    '2025-10-19 10:00:00+00',
    'Studio Photography, South Jakarta',
    3,
    1200000.00,
    'confirmed',
    NOW(),
    NOW()
)
ON CONFLICT (id) DO NOTHING;