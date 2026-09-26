CREATE TABLE projects (
    id UUID PRIMARY KEY,
    client_id UUID NOT NULL,
    name VARCHAR(150) NOT NULL,
    description TEXT,
    project_type PROJECT_TYPE NOT NULL,
    status PROJECT_STATUS NOT NULL DEFAULT 'planning',
    address TEXT,
    city VARCHAR(100) NOT NULL,
    state CHAR(2) NOT NULL,
    start_date DATE,
    estimated_end_date DATE,
    completed_at TIMESTAMPTZ,
    cover_file_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_projects_client FOREIGN KEY (client_id) REFERENCES clients (id) ON DELETE RESTRICT,
    CONSTRAINT fk_projects_cover_file FOREIGN KEY (cover_file_id) REFERENCES files (id) ON DELETE SET NULL,
    CONSTRAINT chk_projects_dates CHECK (
        estimated_end_date IS NULL
        OR start_date IS NULL
        OR estimated_end_date >= start_date
    )
);

CREATE INDEX idx_projects_client_id ON projects (client_id);

CREATE INDEX idx_projects_status ON projects (status);

CREATE INDEX idx_projects_cover_file_id ON projects (cover_file_id);
