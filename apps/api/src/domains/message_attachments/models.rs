use uuid::Uuid;

/// Representa um arquivo anexado a uma mensagem.
/// Conecta-se a Message e File.
/// Usado para permitir o envio de arquivos durante as conversas do projeto.
pub struct MessageAttachment {
    pub message_id: Uuid,
    pub file_id: Uuid,
}
