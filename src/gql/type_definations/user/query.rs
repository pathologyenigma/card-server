use super::types::LoginInput;
use crate::error_handling::{BadInputErrorHandler, ErrorHandler};
use async_graphql::{Context, Error, Object, Result};
use sea_orm::{ColumnTrait, Condition, DbConn, EntityTrait, QueryFilter};
use tracing::{error, info};
#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn log_in(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        info!("Query.UserQuery.logIn accepted one request");
        let db = ctx.data_unchecked::<DbConn>();
        let mut bad_input_error_handler = ctx.data_unchecked::<BadInputErrorHandler>().clone();
        let user = crate::users::Entity::find()
            .filter(
                Condition::any()
                    .add(crate::users::Column::Username.eq(input.account.clone()))
                    .add(crate::users::Column::Email.eq(input.account.clone())),
            )
            .one(db)
            .await
            .expect("failed to query database");
        if let Some(user) = user {
            if user.password == input.password {
                info!("Query.UserQuery.logIn send a response token: fake token");
                return Ok("fake token".to_string());
            } else {
                error!("bad input: wrong password");
                bad_input_error_handler.append("password", "wrong password");
            }
        } else {
            error!("bad input: user not found");
            bad_input_error_handler.append("account", "user not found");
        }
        if !bad_input_error_handler.is_none() {
            return Err(bad_input_error_handler.to_err());
        } else {
            return Err(Error::new("unexpected error"));
        }
    }
}
