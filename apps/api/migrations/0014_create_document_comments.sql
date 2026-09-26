CREATE TABLE document_comments (
    id UUID PRIMARY KEY,
    document_id UUID NOT NULL,
    user_id UUID NOT NULL,
    content TEXT NOT NULL,
    page INTEGER,
    position_x REAL,
    position_y REAL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ,
    CONSTRAINT fk_document_comments_document FOREIGN KEY (document_id) REFERENCES documents (id) ON DELETE CASCADE,
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

CREATE INDEX idx_document_comments_document_id ON document_comments (document_id);

CREATE INDEX idx_document_comments_user_id ON document_comments (user_id);
