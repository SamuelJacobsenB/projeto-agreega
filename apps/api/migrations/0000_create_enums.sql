CREATE TYPE USER_ROLE AS ENUM (
    'admin',
    'engineer',
    'architect',
    'technical',
    'client'
);

CREATE TYPE PROJECT_TYPE AS ENUM (
    'architecture',
    'engineering',
    'architecture_and_engineering',
    'consulting',
    'other'
);

CREATE TYPE PROJECT_STATUS AS ENUM (
    'planning',
    'in_progress',
    'paused',
    'completed',
    'cancelled'
);

CREATE TYPE STAGE_STATUS AS ENUM (
    'pending',
    'in_progress',
    'completed',
    'blocked',
    'cancelled'
);

CREATE TYPE TASK_STATUS AS ENUM (
    'pending',
    'in_progress',
    'completed',
    'blocked',
    'cancelled'
);

CREATE TYPE DOCUMENT_TYPE AS ENUM (
    'contract',
    'project',
    'technical_drawing',
    'report',
    'budget',
    'license',
    'certificate',
    'other'
);

CREATE TYPE DOCUMENT_STATUS AS ENUM (
    'draft',
    'under_review',
    'approved',
    'changes_requested',
    'archived'
);

CREATE TYPE APPROVAL_STATUS AS ENUM (
    'pending',
    'approved',
    'changes_requested',
    'rejected'
);

CREATE TYPE NOTIFICATION_TYPE AS ENUM (
    'general',
    'project',
    'document',
    'approval',
    'message',
    'task',
    'payment',
    'system'
);

CREATE TYPE PAYMENT_STATUS AS ENUM (
    'pending',
    'paid',
    'overdue',
    'cancelled'
);

CREATE TYPE LEAD_STATUS AS ENUM (
    'new',
    'contacted',
    'qualified',
    'proposal',
    'won',
    'lost'
);
