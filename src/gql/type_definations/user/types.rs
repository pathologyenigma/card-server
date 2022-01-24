use async_graphql::InputObject;
#[derive(InputObject)]
pub struct RegisterInput {
    pub(crate) username: String,
    pub(crate) email: Option<String>,
    pub(crate) password: String,
    pub(crate) confirm_password: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub(crate) account: String,
    pub(crate) password: String,
}
