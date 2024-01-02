-- Check if the user does not exist before creating
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT FROM pg_catalog.pg_roles
        WHERE rolname = 'rustic_user'
    ) THEN
        CREATE USER rustic_user WITH PASSWORD 'password';
    END IF;
END
$$;

-- Check if the database does not exist before creating
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT FROM pg_catalog.pg_database
        WHERE datname = 'rustic_db'
    ) THEN
        CREATE DATABASE rustic_db;
    END IF;
END
$$;

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE rustic_db TO rustic_user;

