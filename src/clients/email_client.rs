use std::{sync::Arc, time::Duration};

use lettre::{
    AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, MultiPart, SinglePart},
    transport::smtp::AsyncSmtpTransport,
};

use crate::{
    Error, Result,
    configuration::{app_config::ApplicationConfig, smtp_config::SmtpConfig},
};

#[derive(Debug)]
pub struct EmailClient {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    application_config: ApplicationConfig,
    from: Mailbox,
}

impl EmailClient {
    pub async fn send_account_verification_email(&self, to: &str, token: &str) -> Result<()> {
        let to: Mailbox = to
            .parse()
            .map_err(|_| Error::Conflict("invalid email address".to_string()))?;

        let url = format!(
            "{}{}?email={}&token={}",
            self.application_config.client_url,
            self.application_config.account_verification_path,
            to,
            token
        );

        let text_part = SinglePart::plain(format!(
            "Hello,\n\nTo verify your account on our website, please click the following link:\n\n{}",
            url
        ));

        let html_part = SinglePart::html(format!(
            r#"
            <html>
                <body>
                    <h2>Hello!</h2>
                    <p>To verify your account, please click the link below:</p>
                    <p><a href="{}" style="background-color: #4CAF50; color: white; padding: 15px 32px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px; border-radius: 5px;">Verify Account</a></p>
                    <p>If you did not register on our website, please ignore this message.</p>
                    <br>
                    <p>Best regards,<br>Your Support Team</p>
                </body>
            </html>
            "#,
            url
        ));

        let message = self.build_message(to, "Account Verification", text_part, html_part)?;

        self.transport
            .send(message)
            .await
            .map_err(|e| Error::Internal(format!("Failed to send email: {}", e)))?;

        Ok(())
    }

    pub async fn send_reset_password_email(&self, to: &str, token: &str) -> Result<()> {
        let to: Mailbox = to
            .parse()
            .map_err(|_| Error::Conflict("invalid email address".to_string()))?;

        let url = format!(
            "{}{}?email={}&token={}",
            self.application_config.client_url,
            self.application_config.reset_password_path,
            to,
            token
        );

        let text_part = SinglePart::plain(format!(
            "Hello,\n\nTo reset your password, please click the following link:\n\n{}",
            url
        ));

        let html_part = SinglePart::html(format!(
            r#"
            <html>
                <body>
                    <h2>Hello!</h2>
                    <p>To reset your password, please click the link below:</p>
                    <p><a href="{}" style="background-color: #FF5733; color: white; padding: 15px 32px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px; border-radius: 5px;">Reset Password</a></p>
                    <p>If you did not request a password reset, please ignore this message.</p>
                    <br>
                    <p>Best regards,<br>Your Support Team</p>
                </body>
            </html>
            "#,
            url
        ));

        let message = self.build_message(to, "Password Reset", text_part, html_part)?;

        self.transport
            .send(message)
            .await
            .map_err(|e| Error::Internal(format!("Failed to send email: {}", e)))?;

        Ok(())
    }

    fn build_message(
        &self,
        to: Mailbox,
        subject: &str,
        text_part: SinglePart,
        html_part: SinglePart,
    ) -> Result<Message> {
        let message = Message::builder()
            .from(self.from.to_owned())
            .to(to)
            .subject(subject)
            .multipart(
                MultiPart::mixed()
                    .singlepart(text_part)
                    .singlepart(html_part),
            )?;

        Ok(message)
    }
}

pub async fn build_email_client(
    config: &SmtpConfig,
    application_config: &ApplicationConfig,
) -> Result<Arc<EmailClient>> {
    let credentials = config.credentials();
    let transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
        .map_err(|e| Error::Internal(e.to_string()))?
        .port(config.port)
        .credentials(credentials)
        .timeout(Duration::from_secs(10).into())
        .build();

    transport
        .test_connection()
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    Ok(Arc::new(EmailClient {
        transport,
        application_config: application_config.clone(),
        from: config.from.parse().unwrap(),
    }))
}
