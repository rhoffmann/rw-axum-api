use std::env;

use lettre::{SmtpTransport, message::Mailbox, transport::smtp::authentication::Credentials};

pub struct EmailService {
    mailer: SmtpTransport,
    from_email: Mailbox,
}

impl EmailService {
    pub fn new(&self) -> Result<Self, Box<dyn std::error::Error>> {
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");

        let smtp_port: u16 = env::var("SMPT_PORT")
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
}
