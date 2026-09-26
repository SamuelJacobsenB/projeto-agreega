CREATE TABLE document_approvals (
    id UUID PRIMARY KEY,
    document_id UUID NOT NULL,
    document_version_id UUID NOT NULL,
    requested_by UUID NOT NULL,
    reviewed_by UUID,
    status APPROVAL_STATUS NOT NULL DEFAULT 'pending',
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    reviewed_at TIMESTAMPTZ,
    CONSTRAINT fk_document_approvals_document FOREIGN KEY (document_id) REFERENCES documents (id) ON DELETE CASCADE,
    CONSTRAINT fk_document_approvals_version FOREIGN KEY (document_version_id) REFERENCES document_versions (id) ON DELETE CASCADE,
    CONSTRAINT fk_document_approvals_requested_by FOREIGN KEY (requested_by) REFERENCES users (id) ON DELETE RESTRICT,
    CONSTRAINT fk_document_approvals_reviewed_by FOREIGN KEY (reviewed_by) REFERENCES users (id) ON DELETE
    SET
        NULL,
        CONSTRAINT chk_document_approvals_reviewed_at CHECK (
            status = 'pending'
            OR reviewed_at IS NOT NULL
        )
);

CREATE INDEX idx_document_approvals_document_id ON document_approvals (document_id);

CREATE INDEX idx_document_approvals_document_version_id ON document_approvals (document_version_id);

CREATE INDEX idx_document_approvals_requested_by ON document_approvals (requested_by);

CREATE INDEX idx_document_approvals_reviewed_by ON document_approvals (reviewed_by);

CREATE INDEX idx_document_approvals_status ON document_approvals (status);
