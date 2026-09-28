use super::{dtos::DocumentCommentResponseDto, models::DocumentComment};

impl From<DocumentComment> for DocumentCommentResponseDto {
    fn from(comment: DocumentComment) -> Self {
        Self {
            id: comment.id,
            document_id: comment.document_id,
            user_id: comment.user_id,
            content: comment.content,
            page: comment.page,
            position_x: comment.position_x,
            position_y: comment.position_y,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        }
    }
}
