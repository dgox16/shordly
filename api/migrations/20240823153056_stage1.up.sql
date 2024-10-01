CREATE TABLE urls (
    id_url SERIAL PRIMARY KEY,
    original_url VARCHAR(2048) NOT NULL,
    encoded_id VARCHAR(40),
    counter_visits BIGINT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    visited_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
