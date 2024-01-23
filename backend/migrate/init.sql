CREATE TABLE IF NOT EXISTS users (
  user_id UUID PRIMARY KEY,
  user_name VARCHAR(24) UNIQUE NOT NULL,
  user_password_hash TEXT NOT NULL
);
