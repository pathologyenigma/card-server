use async_graphql::{Object, Result};

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn login(&self) -> String{
        "some fake token".to_string()
    }
}


#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(&self) -> Result<String> {
        Ok("some fake token".to_string())
    }
}