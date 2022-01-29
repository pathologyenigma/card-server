use async_graphql::{MergedObject, MergedSubscription};

mod card;
mod level_setting;
mod user;
#[derive(MergedObject, Default)]
pub struct Query(
    user::query::UserQuery,
    level_setting::query::LevelSettingQuery,
);

#[derive(MergedObject, Default)]
pub struct Mutation(
    user::mutation::UserMutation,
    level_setting::mutation::LevelSettingMutation,
);
#[derive(MergedSubscription, Default)]
pub struct Subscription(level_setting::subscription::LevelSettingSubscription);
