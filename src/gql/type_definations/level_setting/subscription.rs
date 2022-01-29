use async_graphql::{Context, Subscription};
use futures_util::{Stream, StreamExt};
use redis::Client;
use tracing::{error, info};

#[derive(Default)]
pub struct LevelSettingSubscription;

#[Subscription]
impl LevelSettingSubscription {
    async fn get_level_setting(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<impl Stream<Item = super::types::LevelSettingPage>> {
        let token = ctx.data_opt::<crate::TokenFromHeader>();
        match token {
            Some(token) => {
                info!("Subscription.LevelSettingSubscription.getLevelSetting accepted one request with token: {}", token.0);
                let token = crate::Token::decode(
                    token.0.clone(),
                    "just for now, future token will be in a config file".to_string(),
                );
                match token {
                    Ok(token) => {
                        let client = ctx.data_unchecked::<Client>();
                        let mut conn = client.get_async_connection().await.unwrap().into_pubsub();
                        conn.subscribe("level_setting_page").await.unwrap();
                        let user_id = token.id;
                        Ok(conn.into_on_message().filter_map(move |msg| async move {
                            let payload = msg
                                .get_payload::<super::types::LevelSettingPage>()
                                .ok()
                                .unwrap();
                            if payload.user_id != user_id {
                                return None;
                            }
                            Some(payload)
                        }))
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
            None => {
                error!("Subscription.LevelSettingSubscription.getLevelSetting accepted one request without token");
                return Err(crate::new_not_authenticated_error(
                    "miss token in header".to_string(),
                ));
            }
        }
    }
}
