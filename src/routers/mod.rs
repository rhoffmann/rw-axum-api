pub mod auth;
pub mod static_assets;
pub mod user;

pub use auth::auth_routes;
pub use static_assets::create_static_asset_router;
pub use user::user_routes;
