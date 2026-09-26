CREATE TABLE audit_logs (
    id UUID PRIMARY KEY,
    user_id UUID,
    action VARCHAR(100) NOT NULL,
    entity VARCHAR(100) NOT NULL,
    entity_id UUID NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_audit_logs_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE
    SET
        NULL,
        CONSTRAINT chk_audit_logs_action CHECK (LENGTH(TRIM(action)) > 0),
        CONSTRAINT chk_audit_logs_entity CHECK (LENGTH(TRIM(entity)) > 0)
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs (user_id);

CREATE INDEX idx_audit_logs_entity ON audit_logs (entity);

CREATE INDEX idx_audit_logs_entity_id ON audit_logs (entity_id);

CREATE INDEX idx_audit_logs_entity_entity_id ON audit_logs (entity, entity_id);

CREATE INDEX idx_audit_logs_created_at ON audit_logs (created_at);
