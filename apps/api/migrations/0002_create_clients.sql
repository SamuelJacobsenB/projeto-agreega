CREATE TABLE clients (
    id UUID PRIMARY KEY,
    client_type CLIENT_TYPE NOT NULL,
    company_name VARCHAR(150),
    cnpj VARCHAR(14) UNIQUE,
    phone VARCHAR(20),
    email VARCHAR(255) UNIQUE,
    address VARCHAR(255),
    city VARCHAR(100),
    state VARCHAR(2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_clients_company_fields CHECK (
        (
            client_type = 'individual'
            AND company_name IS NULL
            AND cnpj IS NULL
        )
        OR (
            client_type = 'company'
            AND company_name IS NOT NULL
            AND cnpj IS NOT NULL
        )
    ),
    CONSTRAINT chk_clients_company_name CHECK (
        company_name IS NULL
        OR (
            LENGTH(TRIM(company_name)) BETWEEN 3
            AND 150
        )
    ),
    CONSTRAINT chk_clients_email CHECK (
        email IS NULL
        OR email ~* '^[^@\s]+@[^@\s]+\.[^@\s]+$'
    ),
    CONSTRAINT chk_clients_cnpj CHECK (
        cnpj IS NULL
        OR cnpj ~ '^\d{14}$'
    ),
    CONSTRAINT chk_clients_state CHECK (
        state IS NULL
        OR state ~ '^[A-Z]{2}$'
    ),
    CONSTRAINT chk_clients_city CHECK (
        city IS NULL
        OR LENGTH(TRIM(city)) BETWEEN 2
        AND 100
    )
);

ALTER TABLE
    users
ADD
    CONSTRAINT fk_users_client FOREIGN KEY (client_id) REFERENCES clients (id) ON DELETE
SET
    NULL;
