use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateClientRequestDto {
    #[validate(length(
        min = 3,
        max = 150,
        message = "Nome da empresa deve ter entre 3 e 150 caracteres."
    ))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub company_name: Option<String>,

    #[validate(length(min = 14, max = 14, message = "CNPJ deve ter 14 dígitos."))]
    #[validate(custom(function = "crate::validation::validate_cnpj"))]
    pub cnpj: Option<String>,

    #[validate(custom(function = "crate::validation::validate_brazilian_phone"))]
    pub phone: Option<String>,

    #[validate(email(message = "E-mail inválido."))]
    #[validate(length(max = 255, message = "E-mail deve ter no máximo 255 caracteres."))]
    pub email: Option<String>,

    #[validate(length(max = 255, message = "Endereço deve ter no máximo 255 caracteres."))]
    pub address: Option<String>,

    #[validate(length(
        min = 2,
        max = 100,
        message = "Cidade deve ter entre 2 e 100 caracteres."
    ))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub city: Option<String>,

    #[validate(custom(function = "crate::validation::validate_state_code"))]
    pub state: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct UpdateClientRequestDto {
    #[validate(length(min = 3, max = 150))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub company_name: Option<String>,

    #[validate(length(min = 14, max = 14))]
    #[validate(custom(function = "crate::validation::validate_cnpj"))]
    pub cnpj: Option<String>,

    #[validate(custom(function = "crate::validation::validate_brazilian_phone"))]
    pub phone: Option<String>,

    #[validate(email(message = "E-mail inválido."))]
    #[validate(length(max = 255))]
    pub email: Option<String>,

    #[validate(length(max = 255))]
    pub address: Option<String>,

    #[validate(length(min = 2, max = 100))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub city: Option<String>,

    #[validate(custom(function = "crate::validation::validate_state_code"))]
    pub state: Option<String>,
}

#[derive(Serialize)]
pub struct ClientResponseDto {
    pub id: Uuid,
    pub company_name: Option<String>,
    pub cnpj: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
