use super::types::RegisterInput;
use crate::error_handling::{BadInputErrorHandler, ErrorHandlerWithErrorExtensions};
use async_graphql::{Context, Error, Object, Result};
use sea_orm::{entity::Set, DbConn, EntityTrait};
use tracing::{error, info};
#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<String> {
        info!("accepted one request");
        let mut bad_input_error_handler = ctx.data_unchecked::<BadInputErrorHandler>().clone();
        let username = input.username.trim();
        let password = input.password.trim();
        let email = input.email;
        let confirm_password = input.confirm_password.trim();
        if confirm_password != password {
            bad_input_error_handler.append(
                "confirm_password".to_string(),
                "confirm password not match the password".to_string(),
            );
        }
        if bad_input_error_handler.is_none() {
            let user = crate::user::ActiveModel {
                username: Set(username.to_owned()),
                password: Set(crate::pass_hash::hash(password).expect("failed to hash password")),
                email: Set(email.to_owned()),
                ..Default::default()
            };
            let db = ctx.data_unchecked::<DbConn>();
            match crate::user::Entity::insert(user).exec(db).await {
                Ok(res) => {
                    return Ok(
                        crate::Token::new(res.last_insert_id, username.to_string(), email)
                            .encode(
                                "just for now, future token will be in a config file".to_string(),
                            )
                            .expect("failed to pass token"),
                    );
                }
                Err(err) => {
                    error!("{:?}", err);
                    match err {
                        sea_orm::DbErr::Query(err) => {
                            if err.contains("user_username_key") {
                                let msg = format!("username {} is taken", username);
                                bad_input_error_handler.append("username".to_string(), msg);
                                return Err(bad_input_error_handler.to_err());
                            } else if err.contains("user_email_key") {
                                let msg = format!("email {} already binded, you can try login with this email instead", email.unwrap());
                                bad_input_error_handler.append("email".to_string(), msg);
                                return Err(bad_input_error_handler.to_err());
                            } else {
                                return Err(crate::new_internal_server_error(err));
                            }
                        }
                        _ => return Err(Error::new_with_source(err)),
                    }
                }
            }
        }
        return Err(bad_input_error_handler.to_err());
    }
}
