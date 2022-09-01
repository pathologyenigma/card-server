use async_graphql::{Context, Object, Result};
use redis::{AsyncCommands, Client};
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
                error!("accepted one request without token");
                return Err(crate::new_not_authenticated_error(
                    "miss token in header".to_string(),
                ));
            }
        }
    }
    /// get all pages at once
    /// this will send done! when all pages is sent
    /// you may need to subscribe the subscription for geting page data
    async fn get_level_settings(
        &self,
        ctx: &Context<'_>,
        page_size: Option<u64>,
    ) -> Result<String> {
        let token = ctx.data_opt::<crate::TokenFromHeader>();
        match token {
            Some(token) => {
                info!("accepted one request with token: {}", token.0);
                let token = crate::Token::decode(
                    token.0.clone(),
                    "just for now, future token will be in a config file".to_string(),
                );
                match token {
                    Ok(token) => {
                        let db = ctx.data_unchecked::<DbConn>();
                        let page_size = page_size.unwrap_or(10);
                        let client = ctx.data_unchecked::<Client>();
                        let mut conn = client.get_async_connection().await.unwrap();
                        let mut pages = crate::entity::level_settings::Entity::find()
                            .filter(crate::entity::level_settings::Column::UserId.eq(token.id))
                            .paginate(db, page_size as usize);
                        let mut page_num = 1;
                        while let Some(page) = pages.fetch_and_next().await.unwrap() {
                            let data = page
                                .into_iter()
                                .map(|item| super::types::LevelSetting::from(item))
                                .collect::<Vec<super::types::LevelSetting>>();
                            let data = super::types::LevelSettingPage {
                                data,
                                page: page_num,
                                page_size,
                                user_id: token.id,
                            };
                            // rust don't know this is a empty tuple
                            // if you try to remove the let and type specification
                            // you will panic
                            let _: () = conn.publish("level_setting_page", data).await.unwrap();
                            page_num += 1;
                        }
                        return Ok("done!".into());
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
