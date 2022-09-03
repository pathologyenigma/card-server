use super::types::NewCard;
use crate::{traits::prelude::ToModel, BadInputErrorHandler, ErrorHandlerWithErrorExtensions};
use async_graphql::{Context, Error, Object, Result};
use sea_orm::{DbConn, EntityTrait};
use tracing::{error, info};
#[derive(Default)]
pub struct CardMutation;

#[Object]
impl CardMutation {
    async fn new_card(&self, ctx: &Context<'_>, input: NewCard) -> Result<uuid::Uuid> {
        let token = ctx.data_opt::<crate::TokenFromHeader>();
        match token {
            Some(token) => {
                info!("accepted one request with token: {}", token.0);
                let token = crate::Token::decode(
                    token.0.clone(),
                    "just for now, future token will be in a config file".to_string(),
                );
                match token {
                    Ok(user) => {
                        let db = ctx.data_unchecked::<DbConn>();
                        let mut bad_input_error_handler =
                            ctx.data_unchecked::<BadInputErrorHandler>().clone();
                        match input.check_valid(bad_input_error_handler.clone()) {
                            Ok(_) => {
                                match
                                    crate::card::Entity::insert(input.clone().to_model(user.id))
                                        .exec(db)
                                        .await
                                {
                                    Err(err) => {
                                        match err {
                                            sea_orm::DbErr::Query(err) => {
                                                if err.contains("重复键违反唯一约束") {
                                                    let res: Vec<&str> = err.split("\"").collect();
                                                    match res[1] {
                                                            "one_card_name_could_only_exist_one_for_one_user" => {
                                                                let msg = format!("card with name {} already exists", input.name);
                                                                bad_input_error_handler.append("name".to_string(), msg);
                                                            }
                                                            _ => {
                                                                return Err(Error::new_with_source("unknown error"));
                                                            }
                                                        }
                                                } else {
                                                    return Err(Error::new_with_source(err));
                                                }
                                            }
                                            _ => return Err(Error::new_with_source(err)),
                                        }
                                    }
                                    Ok(res) => {
                                        return Ok(res.last_insert_id);
                                    }
                                    
                                }
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            None => {
                error!("accepted one request without token");
                return Err(crate::new_not_authenticated_error(
                    "miss token in header".to_string(),
                ));
            }
        }
        unreachable!("")
    }
}
