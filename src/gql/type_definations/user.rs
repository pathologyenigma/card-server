use async_graphql::{Context, InputObject, Object, Result, Error};
use sea_orm::{ColumnTrait, Condition, DbConn, EntityTrait, QueryFilter, entity::Set};
use tracing::{error, info};
use crate::{EMAIL_VERIFICATION, USERNAME_VERIFICATION, PASSWORD_VERIFICATION};
#[derive(Default)]
pub struct UserQuery;

#[derive(InputObject)]
pub struct LoginInput {
    account: String,
    password: String,
}

#[Object]
impl UserQuery {
    async fn log_in(&self, ctx: &Context<'_>, input: LoginInput) -> Result<String> {
        info!("Query.UserQuery.logIn accepted one request");
        let db = ctx.data_unchecked::<DbConn>();
        let user = crate::users::Entity::find()
        .filter(
            Condition::any()
            .add(crate::users::Column::Username.eq(input.account.clone()))
            .add(crate::users::Column::Email.eq(input.account.clone()))
        ).one(db).await.expect("failed to query database");
        if let Some(user) = user {
            if user.password == input.password {
                info!("Query.UserQuery.logIn send a response token: fake token");
                return Ok("fake token".to_string())
            } else {
                error!("bad input: wrong password");
                return Err(Error::new("bad input: wrong password"))
            }
        } else {
            error!("bad input: user not found");
            Err(Error::new("bad input: user not found"))
        }
    }
    
}

#[derive(Default)]
pub struct UserMutation;

#[derive(InputObject)]
pub struct RegisterInput {
    username: String,
    email: Option<String>,
    password: String,
    confirm_password: String,
}
#[Object]
impl UserMutation {
    
    async fn register(&self, ctx:&Context<'_>, input: RegisterInput) -> Result<String> {
        info!("Mutation.UserMutation.register accepted one request");
        let mut errs = None;
        let username = input.username.trim();
        if username.is_empty() {
            errs = input_err_handle(errs, "username", "empty username is not allowed");
        } else if !USERNAME_VERIFICATION.is_match(username) {
            errs = input_err_handle(errs, "username", "invalid username")
        }
        let password = input.password.trim();
        if password.is_empty() {
            errs = input_err_handle(errs, "password", "empty password is not allowed");
        } else if !PASSWORD_VERIFICATION.is_match(password) {
            errs = input_err_handle(errs, "password", "your password is too weak or the length is not in the range [8,16]");
        }
        let confirm_password = input.confirm_password.trim();
        if confirm_password.is_empty() {
            errs = input_err_handle(errs, "confirm_password", "empty password is not allowed");
        } else if confirm_password != password {
            errs = input_err_handle(errs, "confirm_password", "confirm password not match the password");
        }
        let email = match input.email {
            Some(email) => {
                if !EMAIL_VERIFICATION.is_match(email.trim()) {
                    errs = input_err_handle(errs, "email","not a valid email address");
                    None
                } else {
                    Some(email)
                }
            },
            None => None,
        };
        
        let res;
        match errs {
            Some(errors) => {
                return Err(errors);
            },
            None => {
                
                let user = crate::users::ActiveModel {
                    username: Set(username.to_owned()),
                    password: Set(password.to_owned()),
                    email: Set(email.to_owned()),
                    ..Default::default()
                };
                let db = ctx.data_unchecked::<DbConn>();
                match crate::users::Entity::insert(user).exec(db).await {
                    Ok(_) => res = Ok("fake token".to_string()),
                    Err(err) => {
                        error!("{:?}", err);
                        res = Err(Error::new_with_source(err));
                    }
                }
                
            }
        }
        return res;
        
    }
}
fn input_err_handle(errors: Option<Error>, name: &'static str, value: &'static str) -> Option<Error> {
    if errors.is_none() {
        return super::super::error_handling::append_err(Some(Error::new("bad input")), name, value);
    } else {
        return super::super::error_handling::append_err(errors, name, value);
    }
}