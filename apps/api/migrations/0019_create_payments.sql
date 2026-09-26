CREATE TABLE payments (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL,
    description VARCHAR(255) NOT NULL,
    amount DECIMAL(12, 2) NOT NULL,
    due_date DATE,
    paid_at TIMESTAMPTZ,
    status PAYMENT_STATUS NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_payments_project FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    CONSTRAINT chk_payments_amount CHECK (amount > 0),
    CONSTRAINT chk_payments_description CHECK (LENGTH(TRIM(description)) > 0),
    CONSTRAINT chk_payments_paid_at CHECK (
        status <> 'paid'
        OR paid_at IS NOT NULL
    )
);

CREATE INDEX idx_payments_project_id ON payments (project_id);

CREATE INDEX idx_payments_status ON payments (status);

CREATE INDEX idx_payments_due_date ON payments (due_date);

CREATE INDEX idx_payments_paid_at ON payments (paid_at);
