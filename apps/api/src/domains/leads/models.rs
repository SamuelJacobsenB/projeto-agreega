use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa um potencial cliente interessado nos serviços da Agreega.
/// Não depende de User ou Client.
/// Usado para receber e gerenciar contatos provenientes do site e solicitações de orçamento.
pub struct Lead {
    pub id: Uuid,

    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,

    pub company: Option<String>,

    pub service_type: Option<String>,
    pub description: Option<String>,

    pub city: Option<String>,
    pub state: Option<String>,

    pub status: LeadStatus,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Proposal,
    Won,
    Lost,
}
