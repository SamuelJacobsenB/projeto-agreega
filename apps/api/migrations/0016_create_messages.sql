CREATE TABLE messages (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL,
    sender_id UUID NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    edited_at TIMESTAMPTZ,
    CONSTRAINT fk_messages_project FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    CONSTRAINT fk_messages_sender FOREIGN KEY (sender_id) REFERENCES users (id) ON DELETE RESTRICT,
    CONSTRAINT chk_messages_content CHECK (LENGTH(TRIM(content)) > 0),
    CONSTRAINT chk_messages_edited_at CHECK (
        edited_at IS NULL
        OR edited_at >= created_at
    )
);

CREATE INDEX idx_messages_project_id ON messages (project_id);

CREATE INDEX idx_messages_sender_id ON messages (sender_id);

CREATE INDEX idx_messages_created_at ON messages (created_at);
