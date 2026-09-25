use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa uma solicitação e seu resultado de aprovação de uma versão de documento.
/// Conecta-se a Document e User.
/// Usado quando um documento precisa ser revisado, aprovado ou ter alterações solicitadas.
pub struct DocumentApproval {
    pub id: Uuid,

    pub document_id: Uuid,
    pub version_version_id: Uuid,

    pub requested_by: Uuid,
    pub reviewed_by: Option<Uuid>,

    pub status: ApprovalStatus,
    pub comment: Option<String>,

    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

pub enum ApprovalStatus {
    Pending,
    Approved,
    ChangesRequested,
    Rejected,
}
