// data access layer
//
// The repository has a single responsibility and only handles data access. Testing becomes easier because we can mock // the repository for unit tests. Multiple handlers can reuse the same repository methods, and when we need to change // database queries, we only update them in one place.
pub mod email_verification_repository;
pub mod password_reset_repository;
pub mod traits;
pub mod user_repository;

pub use traits::{
    EmailVerificationRepositoryTrait, PasswordResetRepositoryTrait, UserRepositoryTrait,
};

pub use email_verification_repository::EmailVerificationRepository;
pub use password_reset_repository::PasswordResetRepository;
pub use user_repository::UserRepository;
