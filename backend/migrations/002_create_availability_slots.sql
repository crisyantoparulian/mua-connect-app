-- Create availability_slots table for MUA availability management
CREATE TABLE IF NOT EXISTS availability_slots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    mua_id UUID NOT NULL REFERENCES mua_profiles(id) ON DELETE CASCADE,
    start_time TIME NOT NULL, -- HH:MM format
    end_time TIME NOT NULL,   -- HH:MM format
    day_of_week INTEGER CHECK (day_of_week >= 0 AND day_of_week <= 6), -- 0-6, Sunday to Saturday, NULL for specific dates
    specific_date TIMESTAMP WITH TIME ZONE, -- For one-time availability
    is_available BOOLEAN NOT NULL DEFAULT true,
    recurring BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),

    -- Ensure either day_of_week is set for recurring slots or specific_date for one-time slots
    CONSTRAINT check_recurring_logic CHECK (
        (recurring = true AND day_of_week IS NOT NULL AND specific_date IS NULL) OR
        (recurring = false AND day_of_week IS NULL AND specific_date IS NOT NULL)
    ),

    -- Ensure end_time is after start_time
    CONSTRAINT check_time_order CHECK (end_time > start_time)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_availability_slots_mua_id ON availability_slots(mua_id);
CREATE INDEX IF NOT EXISTS idx_availability_slots_day_of_week ON availability_slots(day_of_week) WHERE day_of_week IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_availability_slots_specific_date ON availability_slots(specific_date) WHERE specific_date IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_availability_slots_is_available ON availability_slots(is_available);

-- Create updated_at trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_availability_slots_updated_at
    BEFORE UPDATE ON availability_slots
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();