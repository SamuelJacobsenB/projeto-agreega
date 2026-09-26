CREATE TABLE project_images (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL,
    file_id UUID NOT NULL,
    "order" SMALLINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_project_images_project FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    CONSTRAINT fk_project_images_file FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE RESTRICT,
    CONSTRAINT chk_project_images_order CHECK ("order" >= 0),
    CONSTRAINT uq_project_images_project_order UNIQUE (project_id, "order"),
    CONSTRAINT uq_project_images_project_file UNIQUE (project_id, file_id)
);

CREATE INDEX idx_project_images_project_id ON project_images (project_id);

CREATE INDEX idx_project_images_file_id ON project_images (file_id);
