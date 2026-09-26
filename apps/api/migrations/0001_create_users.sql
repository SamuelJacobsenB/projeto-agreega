CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role USER_ROLE NOT NULL DEFAULT 'client',
    client_id UUID,
    avatar_file_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT chk_users_email CHECK (
        email ~* '^[^@\s]+@[^@\s]+\.[^@\s]+$'
    ),
);

CREATE INDEX idx_users_client_id ON users (client_id);
