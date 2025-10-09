use uuid::Uuid;

pub fn generate_verification_token() -> String {
    // create token as UUID without hyphens
    Uuid::new_v4().as_simple().to_string()
}
