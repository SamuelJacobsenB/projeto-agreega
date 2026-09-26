CREATE TABLE invitations (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    invited_by UUID NOT NULL,
    client_id UUID,
    role USER_ROLE NOT NULL DEFAULT 'client',
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_invitations_invited_by FOREIGN KEY (invited_by) REFERENCES users (id) ON DELETE RESTRICT,
    CONSTRAINT fk_invitations_client FOREIGN KEY (client_id) REFERENCES clients (id) ON DELETE SET NULL
);
