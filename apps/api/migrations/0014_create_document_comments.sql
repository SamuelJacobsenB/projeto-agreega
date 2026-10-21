CREATE TABLE document_comments (
    id UUID PRIMARY KEY,
    document_version_id UUID NOT NULL,
    user_id UUID NOT NULL,
    content VARCHAR(500) NOT NULL,
    page SMALLINT,
    position_x REAL,
    position_y REAL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ,
    CONSTRAINT fk_document_comments_document_version FOREIGN KEY (document_version_id) REFERENCES document_versions (id) ON DELETE CASCADE,
    CONSTRAINT fk_document_comments_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE RESTRICT,
    CONSTRAINT chk_document_comments_page CHECK (
        page IS NULL
        OR page >= 1
    ),
    CONSTRAINT chk_document_comments_position_x CHECK (
        position_x IS NULL
        OR position_x >= 0
    ),
    CONSTRAINT chk_document_comments_position_y CHECK (
        position_y IS NULL
        OR position_y >= 0
    )
);

CREATE INDEX idx_document_comments_document_version_id ON document_comments (document_version_id);

CREATE INDEX idx_document_comments_user_id ON document_comments (user_id);
