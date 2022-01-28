use async_graphql::{CustomValidator, InputObject, SimpleObject};
struct EmailValidator;
impl CustomValidator<String> for EmailValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if crate::EMAIL_VERIFICATION.is_match(value) {
            return Ok(());
        }
        Err("not a valid email".to_string())
    }
}
struct UsernameValidator;
impl CustomValidator<String> for UsernameValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if crate::USERNAME_VERIFICATION.is_match(value) {
            return Ok(());
        }
        Err("username should be en characters with length between 6~24".to_string())
    }
}
struct PasswordValidator;
impl CustomValidator<String> for PasswordValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if crate::PASSWORD_VERIFICATION.is_match(value) {
            return Ok(());
        }
        Err("password should be en characters with length between 8~16".to_string())
    }
}
struct LoginAccountValidator;
impl CustomValidator<String> for LoginAccountValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if crate::USERNAME_VERIFICATION.is_match(value) || crate::EMAIL_VERIFICATION.is_match(value)
        {
            return Ok(());
        }
        Err("account name should be a valid username or email".to_string())
    }
}
#[derive(InputObject)]
pub struct RegisterInput {
    /// should be en characters with length between 6~24
    /// if not you will got parse error
    #[graphql(validator(custom = "UsernameValidator{}"))]
    pub(crate) username: String,
    /// should be a valid email format
    /// if not you will got parse error
    #[graphql(validator(custom = "EmailValidator{}"))]
    pub(crate) email: Option<String>,
    /// should be en characters with length between 8~16
    /// if not you will got parse error
    #[graphql(validator(custom = "PasswordValidator{}"))]
    pub(crate) password: String,
    /// same as password
    #[graphql(validator(custom = "PasswordValidator{}"))]
    pub(crate) confirm_password: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    /// should be valid username or email
    #[graphql(validator(custom = "LoginAccountValidator{}"))]
    pub(crate) account: String,
    /// same rule with register password
    #[graphql(validator(custom = "PasswordValidator{}"))]
    pub(crate) password: String,
}
#[derive(SimpleObject)]
/// user info
/// email shows out only when user has email
pub struct User {
    pub username: String,
    pub email: Option<String>,
}
