use std::{env, sync::Arc};

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::{
    repositories::{
        EmailVerificationRepository, EmailVerificationRepositoryTrait, PasswordResetRepository,
        PasswordResetRepositoryTrait, RefreshTokenRepository, RefreshTokenRepositoryTrait,
        UserRepository, UserRepositoryTrait,
    },
    services::EmailService,
};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub static_asset_dir: String,
    pub user_repository: Arc<dyn UserRepositoryTrait>,
    pub email_verification_repository: Arc<dyn EmailVerificationRepositoryTrait>,
    pub password_reset_respository: Arc<dyn PasswordResetRepositoryTrait>,
    pub email_service: Arc<EmailService>,
    pub refresh_token_repository: Arc<dyn RefreshTokenRepositoryTrait>,
}

impl AppState {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        // create connection pool
        let db = PgPool::connect(db_url).await?;

        let static_asset_dir =
            env::var("STATIC_ASSET_DIR").unwrap_or_else(|_| "./frontend".to_string());

        // run pending migrations
        sqlx::migrate!("./migrations").run(&db).await?;

        let user_repository: Arc<dyn UserRepositoryTrait> =
            Arc::new(UserRepository::new(db.clone()));

        let email_verification_repository: Arc<dyn EmailVerificationRepositoryTrait> =
            Arc::new(EmailVerificationRepository::new(db.clone()));

        let password_reset_respository: Arc<dyn PasswordResetRepositoryTrait> =
            Arc::new(PasswordResetRepository::new(db.clone()));

        let refresh_token_repository: Arc<dyn RefreshTokenRepositoryTrait> =
            Arc::new(RefreshTokenRepository::new(db.clone()));

        let email_service: Arc<EmailService> = match EmailService::new() {
            Ok(service) => Arc::new(service),
            Err(e) => {
                eprintln!("Failed to initialize email service: {}", e);
                eprintln!("Make sure all SMTP env vars are set in .env");
                panic!("Email service initialization failed");
            }
        };

        Ok(Self {
            db,
            static_asset_dir,
            user_repository,
            email_verification_repository,
            password_reset_respository,
            refresh_token_repository,
            email_service,
        })
    }
}
