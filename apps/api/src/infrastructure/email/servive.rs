use resend_rs::{Resend, types::CreateEmailBaseOptions};

use crate::{
    infrastructure::email::templates,
    response::{AppError, AppResult},
};

#[derive(Clone)]
pub struct EmailService {
    client: Resend,
    from: String,
}

impl EmailService {
    pub fn new(api_key: String, from: String) -> Self {
        Self {
            client: Resend::new(&api_key),
            from,
        }
    }

    pub async fn send(&self, to: &str, subject: &str, html: &str) -> AppResult<()> {
        let email = CreateEmailBaseOptions::new(&self.from, [to], subject).with_html(html);

        self.client
            .emails
            .send(email)
            .await
            .map_err(|_| AppError::Internal("Falha ao enviar e-mail.".to_string()))?;

        Ok(())
    }

    pub async fn send_invitation_email(&self, to: &str, token: &str) -> AppResult<()> {
        let subject = "Convite para Plataforma Agreega";
        let html = templates::invitation(token);

        self.send(to, subject, &html).await
    }

    pub async fn send_password_reset_email(
        &self,
        to: &str,
        app_url: &str,
        token: &str,
    ) -> AppResult<()> {
        let subject = "Redefinição de senha - Plataforma Agreega";
        let html = templates::password_reset(app_url, token);

        self.send(to, subject, &html).await
    }
}
