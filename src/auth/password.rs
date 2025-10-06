use bcrypt::{DEFAULT_COST, hash, verify};

// TODO: what about argon instead of

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    // cost factor 12 is a good balance between performance and security

    hash(password, DEFAULT_COST + 2)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}
