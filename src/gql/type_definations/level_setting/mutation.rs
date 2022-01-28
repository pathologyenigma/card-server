use std::str::FromStr;

use crate::BadInputErrorHandler;
use crate::ErrorHandlerWithErrorExtensions;
use async_graphql::{Context, Object, Result};
use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::DbConn;
use tracing::{error, info};
#[derive(Default)]
pub struct LevelSettingMutation;

#[Object]
impl LevelSettingMutation {
    /// Create a new LevelSetting
    /// you can costomize the type is numberic or text
    /// numberic ones like this 1 star 2 star 3 star and etc.
    /// text ones like this n r sr ssr ur
    async fn add_new_level_setting(
        &self,
        ctx: &Context<'_>,
        input: super::types::NewLevelSetting,
    ) -> Result<super::types::LevelSetting> {
        let token = ctx.data_opt::<crate::TokenFromHeader>();
        match token {
            Some(token) => {
                info!("Mutation.LevelSettingMutation.addNewSetting accepted one request with token: {}", token.0);
                let token = crate::Token::decode(
                    token.0.clone(),
                    "just for now, future token will be in a config file".to_string(),
                );
                match token {
                    Ok(token) => {
                        let db = ctx.data_unchecked::<DbConn>();
                        let mut bad_input_error_handler =
                            ctx.data_unchecked::<BadInputErrorHandler>().clone();
                        let user_id = token.id;
                        let mut prefixed_input = input.clone();
                        let check_result =
                            prefixed_input.check_valid(bad_input_error_handler.clone());
                        if check_result.is_err() {
                            return Err(check_result.unwrap_err());
                        }
                        let level_setting = prefixed_input.clone().to_model(user_id);
                        let res = crate::entity::level_settings::Entity::insert(level_setting)
                            .exec(db)
                            .await;
                        match res {
                            Ok(res) => {
                                let id = res.last_insert_id;
                                return Ok(super::types::LevelSetting {
                                    id: id.to_string(),
                                    title: prefixed_input.title,
                                    is_numberic_level: prefixed_input.is_numberic_level,
                                    levels: prefixed_input.levels,
                                    counts: prefixed_input.counts,
                                    tip_for_setting_user: prefixed_input.tip_for_setting_user,
                                });
                            }
                            Err(err) => match err {
                                sea_orm::DbErr::Query(msg) => {
                                    if msg.contains("重复键违反唯一约束") {
                                        let msg: Vec<&str> = msg.split("\"").collect();
                                        bad_input_error_handler
                                            .append("title".to_string(), msg[1].to_string());
                                        return Err(bad_input_error_handler.to_err());
                                    } else {
                                        return Err(crate::new_internal_server_error(msg));
                                    }
                                }
                                _ => {
                                    return Err(crate::new_internal_server_error(
                                        "unknown error".to_string(),
                                    ))
                                }
                            },
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            None => {
                error!("Mutation.LevelSettingMutation.addNewSetting accepted one request without token");
                return Err(crate::new_not_authenticated_error(
                    "miss token in header".to_string(),
                ));
            }
        }
    }
    /// remove a few settings by their ids
    /// will return the count of the settings
    /// that is actually removed
    async fn remove_level_setting(&self, ctx: &Context<'_>, ids: Vec<String>) -> Result<u64> {
        let token = ctx.data_opt::<crate::TokenFromHeader>();
        match token {
            Some(token) => {
                info!("Mutation.LevelSettingMutation.addNewSetting accepted one request with token: {}", token.0);
                let token = crate::Token::decode(
                    token.0.clone(),
                    "just for now, future token will be in a config file".to_string(),
                );
                match token {
                    Ok(token) => {
                        let db = ctx.data_unchecked::<DbConn>();
                        let res = crate::entity::level_settings::Entity::delete_many()
                            .filter(
                                crate::entity::level_settings::Column::Id.is_in(
                                    ids.into_iter()
                                        .map(|id| {
                                            return uuid::Uuid::from_str(id.as_str()).unwrap();
                                        })
                                        .collect::<Vec<uuid::Uuid>>(),
                                ),
                            )
                            .exec(db)
                            .await
                            .unwrap()
                            .rows_affected;
                        return Ok(res);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            None => {
                error!("Mutation.LevelSettingMutation.remove_level_setting accepted one request without token");
                return Err(crate::new_not_authenticated_error(
                    "miss token in header".to_string(),
                ));
            }
        }
    }
}
