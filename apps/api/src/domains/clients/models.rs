use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

/// Representa o cliente da Agreega, pessoa ou empresa responsável por um ou mais projetos.
/// Conecta-se a User e Project.
/// Usado para identificar o responsável pelos projetos e controlar o acesso do cliente.
#[derive(sqlx::FromRow)]
pub struct Client {
    pub id: Uuid,

    pub client_type: ClientType,

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "client_type", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum ClientType {
    Individual,
    Company,
}
