/// This includes our Schemas / DTOs are structs that define the shape of data coming into and going out of our API.
/// They’re different from our database models because API requests might not include all fields (like no ID when
/// creating), API responses might exclude sensitive fields (like password_hash), and validation happens on DTOs,
/// not database models (because this is the structure that we are using for request and response, the database model
/// structure is used for storage and retrieval of data)
pub mod user_schemas;
