use async_graphql::{Context, Object};
use sea_orm::EntityTrait;
use tracing::{error, info};
use crate::BadInputErrorHandler;
use sea_orm::DbConn;
#[derive(Default)]
pub struct LevelSettingMutation;

#[Object]
impl LevelSettingMutation {
    async fn add_new_level_setting(
        &self,
        ctx: &Context<'_>,
        input: super::types::NewLevelSetting,
    ) -> async_graphql::Result<super::types::LevelSetting> {
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
                        let bad_input_error_handler =
                            ctx.data_unchecked::<BadInputErrorHandler>().clone();
                        let user_id = token.id;
                        let mut prefixed_input = input.clone();
                        let check_result = prefixed_input.check_valid(bad_input_error_handler);
                        if check_result.is_err() {
                            return Err(check_result.unwrap_err());
                        }
                        let level_setting = prefixed_input.clone().to_model(user_id);
                        let res = crate::entity::level_settings::Entity::insert(level_setting)
                            .exec(db)
                            .await
                            .unwrap();
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
}
