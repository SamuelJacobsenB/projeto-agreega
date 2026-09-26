CREATE TABLE stages (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL,
    name VARCHAR(150) NOT NULL,
    description TEXT,
    weight DECIMAL (10, 4) NOT NULL DEFAULT 1,
    "order" SMALLINT NOT NULL,
    status STAGE_STATUS NOT NULL DEFAULT 'pending',
    start_date DATE,
    end_date DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_stages_project FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    CONSTRAINT chk_stages_weight CHECK (weight > 0),
    CONSTRAINT chk_stages_order CHECK ("order" >= 0),
    CONSTRAINT chk_stages_dates CHECK (
        end_date IS NULL
        OR start_date IS NULL
        OR end_date >= start_date
    ),
    CONSTRAINT uq_stages_project_order UNIQUE (project_id, "order")
);

CREATE INDEX idx_stages_project_id ON stages (project_id);

CREATE INDEX idx_stages_status ON stages (status);
