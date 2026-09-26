CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    stage_id UUID NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    weight DECIMAL(10, 4) NOT NULL DEFAULT 1,
    progress DECIMAL(5, 4) NOT NULL DEFAULT 0,
    "order" SMALLINT NOT NULL,
    status TASK_STATUS NOT NULL DEFAULT 'pending',
    assigned_to UUID,
    due_date DATE,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_tasks_stage FOREIGN KEY (stage_id) REFERENCES stages (id) ON DELETE CASCADE,
    CONSTRAINT fk_tasks_assigned_to FOREIGN KEY (assigned_to) REFERENCES users (id) ON DELETE
    SET
        NULL,
        CONSTRAINT chk_tasks_weight CHECK (weight > 0),
        CONSTRAINT chk_tasks_progress CHECK (
            progress >= 0
            AND progress <= 1
        ),
        CONSTRAINT chk_tasks_order CHECK ("order" >= 0),
        CONSTRAINT chk_tasks_completed_at CHECK (
            status <> 'completed'
            OR completed_at IS NOT NULL
        ),
        CONSTRAINT chk_tasks_dates CHECK (
            due_date IS NULL
            OR due_date >= CURRENT_DATE
        ),
        CONSTRAINT uq_tasks_stage_order UNIQUE (stage_id, "order")
);

CREATE INDEX idx_tasks_stage_id ON tasks (stage_id);

CREATE INDEX idx_tasks_assigned_to ON tasks (assigned_to);

CREATE INDEX idx_tasks_status ON tasks (status);

CREATE INDEX idx_tasks_due_date ON tasks (due_date);
