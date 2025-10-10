use std::env;

use chrono::{Datelike, Local};
use lettre::{
    Message, SmtpTransport, Transport,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};

pub struct EmailService {
    mailer: SmtpTransport,
    from_email: Mailbox,
}

impl EmailService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");

        let smtp_port: u16 = env::var("SMTP_PORT")
            .expect("SMTP_PORT must be set")
            .parse()
            .expect("SMTP_PORT must be a valid number");

        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

        let from_email = env::var("SMTP_FROM_EMAIL").expect("SMTP_FROM_EMAIL must be set");
        let from_name = env::var("SMTP_FROM_NAME").expect("SMTP_FROM_NAME must be set");

        let credentials = Credentials::new(smtp_username, smtp_password);

        let mailer = SmtpTransport::starttls_relay(&smtp_host)?
            .port(smtp_port)
            .credentials(credentials)
            .build();

        let from_email = format!("{} <{}>", from_name, from_email)
            .parse::<Mailbox>()
            .expect("Invalid from email format");

        Ok(Self { mailer, from_email })
    }

    pub async fn send_verification_email(
        &self,
        to_email: &str,
        username: &str,
        verification_token: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: probably check some HTML templating engine for this.
        let base_var = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        let verification_link = format!(
            "{}/api/auth/verify-email?token={}",
            base_var, verification_token
        );

        let app_name = env::var("APP_NAME").unwrap_or_else(|_| "MyApp".to_string());
        let current_year = Local::now().date_naive().year().to_string();

        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Welcome to {}!</h1>
                    </div>
                    <div class="content">
                        <h2>Hi {}!</h2>
                        <p>Thanks for signing up! We're excited to have you on board.</p>
                        <p>Please verify your email address by clicking the button below:</p>
                        <div style="text-align: center;">
                            <a href="{}" class="button">Verify Email Address</a>
                        </div>
                        <p>Or copy and paste this link into your browser:</p>
                        <p style="background-color: #eee; padding: 10px; word-break: break-all;">{}</p>
                        <p><strong>This link will expire in 24 hours.</strong></p>
                        <p>If you didn't create an account, please ignore this email.</p>
                    </div>
                    <div class="footer">
                        <p>© {} {}. All rights reserved.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            app_name, username, verification_link, verification_link, current_year, app_name
        );

        let email = Message::builder()
            .from(self.from_email.clone())
            .to(to_email.parse()?)
            .subject("Verify your Email address")
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        self.mailer.send(&email)?;

        println!("Verification email sent to {}", to_email);
        println!("Verification link: {}", verification_link);

        Ok(())
    }
}
