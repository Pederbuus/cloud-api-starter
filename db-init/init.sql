-- Use the pgcrypto extension to generate UUIDs
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create a table with a UUID primary key
CREATE TABLE vehicles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    make VARCHAR(255) NOT NULL,
    model VARCHAR(255) NOT NULL,
    year SMALLINT NOT NULL
);

-- Insert some initial data
INSERT INTO vehicles (make, model, year) VALUES
('Toyota', 'Camry', 2022),
('Honda', 'Civic', 2023),
('Ford', 'F-150', 2024);