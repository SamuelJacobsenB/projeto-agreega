CREATE TABLE files (
    id UUID PRIMARY KEY,
    storage_key TEXT NOT NULL,
    original_name VARCHAR(255) NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    size_bytes BIGINT NOT NULL,
    checksum VARCHAR(128),
    uploaded_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_files_uploaded_by FOREIGN KEY (uploaded_by) REFERENCES users (id) ON DELETE RESTRICT,
    CONSTRAINT chk_files_size CHECK (size_bytes >= 0)
);

CREATE INDEX idx_files_uploaded_by ON files (uploaded_by);

ALTER TABLE users ADD CONSTRAINT fk_users_avatar_file FOREIGN KEY (avatar_file_id) REFERENCES files (id) ON DELETE SET NULL;
