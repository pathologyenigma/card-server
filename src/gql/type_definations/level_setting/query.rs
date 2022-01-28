use crate::BadInputErrorHandler;
use crate::ErrorHandlerWithErrorExtensions;
use async_graphql::{Context, Object, Result};
use sea_orm::entity::*;
use sea_orm::query::*;
use sea_orm::DbConn;
use tracing::{error, info};
#[derive(Default)]
pub struct LevelSettingQuery;
#[Object]
impl LevelSettingQuery {
    async fn get_level_settings_page_by_page(
        &self,
        ctx: &Context<'_>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<Vec<super::types::LevelSetting>> {
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
                        let page = page.unwrap_or(0);
                        let page_size = page_size.unwrap_or(10);
                        let res = crate::entity::level_settings::Entity::find()
                            .filter(crate::entity::level_settings::Column::UserId.eq(token.id))
                            .limit(page_size)
                            .offset(page * page_size)
                            .all(db)
                            .await
                            .unwrap();
                        return Ok(res
                            .into_iter()
                            .map(|item| super::types::LevelSetting::from(item))
                            .collect::<Vec<super::types::LevelSetting>>());
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
    async fn get_level_settings(
        &self,
        ctx: &Context<'_>,
        page_size: Option<u64>,
    ) -> Result<Vec<Vec<super::types::LevelSetting>>> {
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
                        let page_size = page_size.unwrap_or(10);
                        let mut pages = crate::entity::level_settings::Entity::find()
                            .filter(crate::entity::level_settings::Column::UserId.eq(token.id))
                            .paginate(db, page_size as usize);
                        let mut res = Vec::new();
                        while let Some(page) = pages.fetch_and_next().await.unwrap() {
                            res.push(
                                page.into_iter()
                                    .map(|item| super::types::LevelSetting::from(item))
                                    .collect::<Vec<super::types::LevelSetting>>(),
                            );
                        }
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
