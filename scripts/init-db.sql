-- Initialize Avila Analytics GA4 Database Schema
-- PostgreSQL 14+

-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- Events table (main storage)
CREATE TABLE IF NOT EXISTS events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    measurement_id VARCHAR(50) NOT NULL,
    client_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255),
    session_id UUID NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    event_name VARCHAR(100) NOT NULL,
    event_params JSONB,
    user_properties JSONB,
    device_category VARCHAR(50),
    browser VARCHAR(100),
    browser_version VARCHAR(50),
    os VARCHAR(100),
    os_version VARCHAR(50),
    country VARCHAR(2),
    region VARCHAR(100),
    city VARCHAR(100),
    language VARCHAR(10),
    screen_resolution VARCHAR(20),
    viewport_size VARCHAR(20),
    page_location TEXT,
    page_referrer TEXT,
    page_title TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    measurement_id VARCHAR(50) NOT NULL,
    client_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255),
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ,
    events_count INTEGER DEFAULT 0,
    duration_seconds INTEGER,
    landing_page TEXT,
    exit_page TEXT,
    referrer TEXT,
    campaign_source VARCHAR(100),
    campaign_medium VARCHAR(100),
    campaign_name VARCHAR(100),
    device_category VARCHAR(50),
    country VARCHAR(2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    measurement_id VARCHAR(50) NOT NULL,
    client_id VARCHAR(255) NOT NULL UNIQUE,
    user_id VARCHAR(255) UNIQUE,
    first_seen_at TIMESTAMPTZ NOT NULL,
    last_seen_at TIMESTAMPTZ NOT NULL,
    sessions_count INTEGER DEFAULT 1,
    events_count INTEGER DEFAULT 0,
    total_engagement_seconds INTEGER DEFAULT 0,
    properties JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Conversions table
CREATE TABLE IF NOT EXISTS conversions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    measurement_id VARCHAR(50) NOT NULL,
    client_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255),
    session_id UUID NOT NULL,
    event_id UUID NOT NULL REFERENCES events(id),
    conversion_name VARCHAR(100) NOT NULL,
    conversion_value DECIMAL(10, 2),
    currency VARCHAR(3),
    timestamp TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_events_measurement_id ON events(measurement_id);
CREATE INDEX idx_events_client_id ON events(client_id);
CREATE INDEX idx_events_session_id ON events(session_id);
CREATE INDEX idx_events_timestamp ON events(timestamp);
CREATE INDEX idx_events_event_name ON events(event_name);
CREATE INDEX idx_events_params_gin ON events USING GIN(event_params);

CREATE INDEX idx_sessions_measurement_id ON sessions(measurement_id);
CREATE INDEX idx_sessions_client_id ON sessions(client_id);
CREATE INDEX idx_sessions_started_at ON sessions(started_at);

CREATE INDEX idx_users_measurement_id ON users(measurement_id);
CREATE INDEX idx_users_client_id ON users(client_id);
CREATE INDEX idx_users_user_id ON users(user_id);
CREATE INDEX idx_users_last_seen_at ON users(last_seen_at);

CREATE INDEX idx_conversions_measurement_id ON conversions(measurement_id);
CREATE INDEX idx_conversions_client_id ON conversions(client_id);
CREATE INDEX idx_conversions_timestamp ON conversions(timestamp);

-- Partitioning for events table (by month)
-- Uncomment for production with large data volumes
-- CREATE TABLE events_2024_01 PARTITION OF events
--     FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');

-- Views for common queries
CREATE OR REPLACE VIEW daily_stats AS
SELECT
    DATE(timestamp) as date,
    measurement_id,
    COUNT(*) as events,
    COUNT(DISTINCT session_id) as sessions,
    COUNT(DISTINCT client_id) as users
FROM events
GROUP BY DATE(timestamp), measurement_id;

CREATE OR REPLACE VIEW top_pages AS
SELECT
    measurement_id,
    page_location,
    COUNT(*) as pageviews,
    COUNT(DISTINCT session_id) as unique_sessions
FROM events
WHERE event_name = 'page_view'
GROUP BY measurement_id, page_location
ORDER BY pageviews DESC;

-- Grant permissions
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO postgres;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO postgres;
