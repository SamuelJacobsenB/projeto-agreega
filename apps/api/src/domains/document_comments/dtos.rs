use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateDocumentCommentRequestDto {
    #[validate(length(
        min = 1,
        max = 500,
        message = "Mensagem deve ter entre 1 e 500 caracteres."
    ))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub content: String,

    #[validate(range(min = 1, message = "A página deve ser maior ou igual a 1."))]
    pub page: Option<i16>,
    #[validate(range(min = 0.0, message = "A posição X não pode ser negativa."))]
    pub position_x: Option<f32>,
    #[validate(range(min = 0.0, message = "A posição Y não pode ser negativa."))]
    pub position_y: Option<f32>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateDocumentCommentRequestDto {
    #[validate(length(
        min = 1,
        max = 500,
        message = "Mensagem deve ter entre 1 e 500 caracteres."
    ))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub content: Option<String>,

    #[validate(range(min = 1, message = "A página deve ser maior ou igual a 1."))]
    pub page: Option<i16>,
    #[validate(range(min = 0.0, message = "A posição X não pode ser negativa."))]
    pub position_x: Option<f32>,
    #[validate(range(min = 0.0, message = "A posição Y não pode ser negativa."))]
    pub position_y: Option<f32>,
}

#[derive(Serialize)]
pub struct DocumentCommentResponseDto {
    pub id: Uuid,
    pub document_version_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub page: Option<i16>,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
