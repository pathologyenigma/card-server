use async_graphql::MergedObject;

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
pub struct Subscription;
