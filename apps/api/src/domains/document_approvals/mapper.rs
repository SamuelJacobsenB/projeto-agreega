use super::{dtos::DocumentApprovalResponseDto, models::DocumentApproval};

impl From<DocumentApproval> for DocumentApprovalResponseDto {
    fn from(document_approval: DocumentApproval) -> Self {
        DocumentApprovalResponseDto {
            id: document_approval.id,
            document_id: document_approval.document_id,
            document_version_id: document_approval.document_version_id,
            requested_by: document_approval.requested_by,
            reviewed_by: document_approval.reviewed_by,
            status: document_approval.status,
            comment: document_approval.comment,
            created_at: document_approval.created_at,
            reviewed_at: document_approval.reviewed_at,
        }
    }
}
