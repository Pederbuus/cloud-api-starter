-- Use the pgcrypto extension to generate UUIDs
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create a table with a UUID primary key
CREATE TABLE vehicle (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    make VARCHAR(255) NOT NULL,
    model VARCHAR(255) NOT NULL,
    year INTEGER NOT NULL CHECK (year >= 0)
    );

-- Insert some initial data
-- Using fixed UUIDs for consistency in testing
INSERT INTO vehicle (id, make, model, year) VALUES
('00000000-0000-0000-0000-000000000000', 'Toyota', 'Camry', 2022),
('00000000-0000-0000-0000-000000000001', 'Honda', 'Civic', 2023),
('00000000-0000-0000-0000-000000000002', 'Ford', 'F-150', 2024);