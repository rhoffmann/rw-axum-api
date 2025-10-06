// database entity definitions

// A model is a Rust struct that mirrors our database table structure.
// It’s the bridge between our SQL database and our Rust application.
pub mod user;

// This allows other parts of the application to import simply: use crate::models::User; instead of crate::models::user::User.
pub use user::User;
