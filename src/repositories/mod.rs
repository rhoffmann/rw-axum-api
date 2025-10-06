// data access layer
//
// The repository has a single responsibility and only handles data access. Testing becomes easier because we can mock // the repository for unit tests. Multiple handlers can reuse the same repository methods, and when we need to change // database queries, we only update them in one place.
pub mod user_repository;

pub use user_repository::UserRepository;
