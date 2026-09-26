CREATE TABLE clients (
    id UUID PRIMARY KEY,
    company_name VARCHAR(150),
    document VARCHAR(20),
    phone VARCHAR(20),
    email VARCHAR(255) UNIQUE,
    address VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT chk_clients_email CHECK (
        email IS NULL
        OR email ~* '^[^@\s]+@[^@\s]+\.[^@\s]+$'
    ),
    CONSTRAINT chk_clients_state CHECK (
        state IS NULL
        OR state ~ '^[A-Z]{2}$'
    )
);

ALTER TABLE
    users
ADD
    CONSTRAINT fk_users_client FOREIGN KEY (client_id) REFERENCES clients (id) ON DELETE
SET
    NULL;
