use crate::{config::Config, error::{ApiError, ApiResult}};

#[derive(Clone)]
pub struct EmailService {
    from: String,
    to: String,
    api_key: String,
}

impl EmailService {
    pub fn new(config: &Config) -> Self {
        Self {
            from: config.email_from.clone(),
            to: config.email_to.clone(),
            api_key: config.email_api_key.clone(),
        }
    }
    
    pub async fn send_quote_notification(
        &self,
        company_name: &str,
        contact_name: &str,
        email: &str,
        phone: Option<&str>,
        products: &str,
        message: Option<&str>,
    ) -> ApiResult<()> {
        let body = format!(
            r#"
Nueva Solicitud de Cotización - LabMedical

Empresa: {}
Contacto: {}
Email: {}
Teléfono: {}

Productos de interés:
{}

Mensaje adicional:
{}

---
Este es un mensaje automático del sistema de cotizaciones de LabMedical.
            "#,
            company_name,
            contact_name,
            email,
            phone.unwrap_or("No proporcionado"),
            products,
            message.unwrap_or("Ninguno")
        );
        
        self.send_email(
            &format!("Nueva Cotización de {}", company_name),
            &body,
        ).await
    }
    
    async fn send_email(&self, subject: &str, body: &str) -> ApiResult<()> {
        // Para Resend, usamos su API directamente vía HTTP
        // Esta es una versión simplificada - en producción, usar el crate resend-rs
        let client = reqwest::Client::new();
        
        let response = client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "from": self.from,
                "to": [self.to],
                "subject": subject,
                "text": body,
            }))
            .send()
            .await
            .map_err(|e| ApiError::Internal(format!("Error al enviar email: {}", e)))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Error desconocido".to_string());
            return Err(ApiError::Internal(format!("Error del servicio de email: {}", error_text)));
        }
        
        Ok(())
    }
}
