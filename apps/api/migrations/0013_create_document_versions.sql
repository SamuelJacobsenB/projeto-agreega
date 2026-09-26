CREATE TABLE document_versions (
    id UUID PRIMARY KEY,
    document_id UUID NOT NULL,
    file_id UUID NOT NULL,
    version SMALLINT NOT NULL,
    uploaded_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_document_versions_document FOREIGN KEY (document_id) REFERENCES documents (id) ON DELETE CASCADE,
    CONSTRAINT fk_document_versions_file FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE RESTRICT,
    CONSTRAINT fk_document_versions_uploaded_by FOREIGN KEY (uploaded_by) REFERENCES users (id) ON DELETE RESTRICT,
    CONSTRAINT chk_document_versions_version CHECK (version >= 1),
    CONSTRAINT uq_document_versions_document_version UNIQUE (document_id, version)
);

CREATE INDEX idx_document_versions_document_id ON document_versions (document_id);

CREATE INDEX idx_document_versions_file_id ON document_versions (file_id);

CREATE INDEX idx_document_versions_uploaded_by ON document_versions (uploaded_by);
