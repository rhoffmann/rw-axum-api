use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::{EmailVerificationToken, User};

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, sqlx::Error>;

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error>;

    async fn update(
        &self,
        id: Uuid,
        username: Option<&str>,
        email: Option<&str>,
        bio: Option<&str>,
        image: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;
}

#[async_trait]
pub trait EmailVerificationRepositoryTrait: Send + Sync {
    async fn create_token(
        &self,
        user_id: Uuid,
        token: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<EmailVerificationToken, sqlx::Error>;

    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<EmailVerificationToken>, sqlx::Error>;

    async fn delete_token(&self, token: &str) -> Result<(), sqlx::Error>;

    async fn verify_user_email(&self, user_id: Uuid) -> Result<(), sqlx::Error>;
}
