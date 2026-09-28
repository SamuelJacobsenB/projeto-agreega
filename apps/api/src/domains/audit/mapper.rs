use super::{dtos::AuditLogResponseDto, models::AuditLog};

impl From<AuditLog> for AuditLogResponseDto {
    fn from(audit_log: AuditLog) -> Self {
        Self {
            id: audit_log.id,
            user_id: audit_log.user_id,
            action: audit_log.action,
            entity: audit_log.entity,
            entity_id: audit_log.entity_id,
            metadata: audit_log.metadata,
            created_at: audit_log.created_at,
        }
    }
}
