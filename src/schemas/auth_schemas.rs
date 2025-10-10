use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub user: RegisterUserData,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserData {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,

    #[validate(email(message = "Invalid Email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserRequest {
    pub user: LoginUserData,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserData {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: UserData,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub email: String,
    pub username: String,
    pub token: String,         // always add jwt token
    pub bio: String,           // empty string if none in db
    pub image: Option<String>, // null in json if none
    pub email_verified: bool,
}

impl UserData {
    pub fn from_user_with_token(user: crate::models::User, token: String) -> Self {
        Self {
            email: user.email,
            token,
            username: user.username,
            bio: user.bio.unwrap_or_default(),
            image: user.image,
            email_verified: user.email_verified,
        }
    }
}
