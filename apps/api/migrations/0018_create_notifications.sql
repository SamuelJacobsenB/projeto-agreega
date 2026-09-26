CREATE TABLE notifications (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    title VARCHAR(100) NOT NULL,
    message TEXT NOT NULL,
    notification_type NOTIFICATION_TYPE NOT NULL,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_notifications_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT chk_notifications_title CHECK (LENGTH(TRIM(title)) > 0),
    CONSTRAINT chk_notifications_message CHECK (LENGTH(TRIM(message)) > 0),
    CONSTRAINT chk_notifications_read_at CHECK (
        read_at IS NULL
        OR read_at >= created_at
    )
);

CREATE INDEX idx_notifications_user_id ON notifications (user_id);

CREATE INDEX idx_notifications_notification_type ON notifications (notification_type);

CREATE INDEX idx_notifications_read_at ON notifications (read_at);

CREATE INDEX idx_notifications_created_at ON notifications (created_at);
