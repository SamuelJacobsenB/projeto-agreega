CREATE TABLE message_attachments (
    message_id UUID NOT NULL,
    file_id UUID NOT NULL,
    CONSTRAINT pk_message_attachments PRIMARY KEY (message_id, file_id),
    CONSTRAINT fk_message_attachments_message FOREIGN KEY (message_id) REFERENCES messages (id) ON DELETE CASCADE,
    CONSTRAINT fk_message_attachments_file FOREIGN KEY (file_id) REFERENCES files (id) ON DELETE RESTRICT
);

CREATE INDEX idx_message_attachments_file_id ON message_attachments (file_id);
