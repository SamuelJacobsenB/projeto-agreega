CREATE TABLE photos (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL,
    stage_id UUID NOT NULL,
    task_id UUID,
    file_id UUID NOT NULL,
    description TEXT,
    uploaded_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_photos_project FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    CONSTRAINT fk_photos_stage FOREIGN KEY (stage_id) REFERENCES stages (id) ON DELETE CASCADE,
    CONSTRAINT fk_photos_task FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE
    SET
        NULL,
        CONSTRAINT fk_photos_file FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE RESTRICT,
        CONSTRAINT fk_photos_uploaded_by FOREIGN KEY (uploaded_by) REFERENCES users (id) ON DELETE RESTRICT
);

CREATE INDEX idx_photos_project_id ON photos (project_id);

CREATE INDEX idx_photos_stage_id ON photos (stage_id);

CREATE INDEX idx_photos_task_id ON photos (task_id);

CREATE INDEX idx_photos_file_id ON photos (file_id);

CREATE INDEX idx_photos_uploaded_by ON photos (uploaded_by);
