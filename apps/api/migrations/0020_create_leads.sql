CREATE TABLE leads (
    id UUID PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(30),
    company VARCHAR(150),
    service_type VARCHAR(100),
    description TEXT,
    city VARCHAR(100),
    state VARCHAR(2),
    status LEAD_STATUS NOT NULL DEFAULT 'new',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_leads_name CHECK (LENGTH(TRIM(name)) > 0),
    CONSTRAINT chk_leads_email CHECK (
        email IS NULL
        OR email ~* '^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$'
    ),
    CONSTRAINT chk_leads_state CHECK (
        state IS NULL
        OR state ~ '^[A-Z]{2}$'
    )
);

CREATE INDEX idx_leads_email ON leads (email);

CREATE INDEX idx_leads_phone ON leads (phone);

CREATE INDEX idx_leads_status ON leads (status);

CREATE INDEX idx_leads_created_at ON leads (created_at);
