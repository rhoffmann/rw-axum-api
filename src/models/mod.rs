/// A model is a Rust struct that mirrors our database table structure.
/// It’s the bridge between our SQL database and our Rust application.
///
pub mod user;

pub use user::User;
