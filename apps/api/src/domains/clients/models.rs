use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa o cliente da Agreega, pessoa ou empresa responsável por um ou mais projetos.
/// Conecta-se a User e Project.
/// Usado para identificar o responsável pelos projetos e controlar o acesso do cliente.
pub struct Client {
    pub id: Uuid,

    pub company_name: Option<String>,
    pub document: Option<String>,

    pub phone: Option<String>,
    pub email: Option<String>,

    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
