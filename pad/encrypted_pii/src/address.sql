CREATE TABLE user_data (
    user_id SERIAL PRIMARY KEY,
    encrypted_address BYTEA NOT NULL,
    bloom_filter BYTEA NOT NULL
);
