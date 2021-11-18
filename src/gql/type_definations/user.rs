use async_graphql::{Context, InputObject, Object, Result};
use sea_orm::{ColumnTrait, Condition, DbConn, EntityTrait, QueryFilter};

#[derive(Default)]
pub struct UserQuery;

#[derive(InputObject)]
pub struct LoginInput {
    account: String,
    password: String,
}

#[Object]
impl UserQuery {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        let db = ctx.data_unchecked::<DbConn>();
        let user = crate::users::Entity::find().filter(
             Condition::any()
            .add(crate::users::Column::Username.eq(input.account.clone()))
            .add(crate::users::Column::Email.eq(input.account.clone()))
        ).one(db).await.expect("failed to query database");
        if let Some(user) = user {
            if user.password == input.password {
                return Ok("fake token".to_string())
            } else {
                return Err(async_graphql::Error::new("bad input: wrong password"))
            }
        } else {
            Err(async_graphql::Error::new("user not found"))
        }
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
