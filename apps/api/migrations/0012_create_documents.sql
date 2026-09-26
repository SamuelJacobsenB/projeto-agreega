CREATE TABLE documents (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL,
    stage_id UUID NOT NULL,
    task_id UUID,
    name VARCHAR(200) NOT NULL,
    document_type DOCUMENT_TYPE NOT NULL,
    status DOCUMENT_STATUS NOT NULL DEFAULT 'draft',
    current_version SMALLINT NOT NULL DEFAULT 1,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_documents_project FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    CONSTRAINT fk_documents_stage FOREIGN KEY (stage_id) REFERENCES stages (id) ON DELETE CASCADE,
    CONSTRAINT fk_documents_task FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE
    SET
        NULL,
        CONSTRAINT fk_documents_created_by FOREIGN KEY (created_by) REFERENCES users (id) ON DELETE RESTRICT,
        CONSTRAINT chk_documents_current_version CHECK (current_version >= 1)
);

CREATE INDEX idx_documents_project_id ON documents (project_id);

CREATE INDEX idx_documents_stage_id ON documents (stage_id);

CREATE INDEX idx_documents_task_id ON documents (task_id);

CREATE INDEX idx_documents_document_type ON documents (document_type);

CREATE INDEX idx_documents_status ON documents (status);

CREATE INDEX idx_documents_created_by ON documents (created_by);
