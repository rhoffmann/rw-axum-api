use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use crate::{
    repositories::{
        EmailVerificationRepository, EmailVerificationRepositoryTrait, UserRepository,
        UserRepositoryTrait,
    },
    services::EmailService,
};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: PgPool,
    pub user_repository: Arc<dyn UserRepositoryTrait>,
    pub email_verification_repository: Arc<dyn EmailVerificationRepositoryTrait>,
    pub email_service: Arc<EmailService>,
}

impl AppState {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        // create connection pool
        let db = PgPool::connect(db_url).await?;

        // run pending migrations
        sqlx::migrate!("./migrations").run(&db).await?;

        let user_repository: Arc<dyn UserRepositoryTrait> =
            Arc::new(UserRepository::new(db.clone()));

        let email_verification_repository: Arc<dyn EmailVerificationRepositoryTrait> =
            Arc::new(EmailVerificationRepository::new(db.clone()));

        let email_service: Arc<EmailService> =
            Arc::new(EmailService::new().expect("Failed to initialize email service"));

        Ok(Self {
            db,
            user_repository,
            email_verification_repository,
            email_service,
        })
    }
}
