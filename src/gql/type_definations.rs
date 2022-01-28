use async_graphql::MergedObject;

mod user;
pub use user::types as user_types;
mod level_setting;
#[derive(MergedObject, Default)]
pub struct Query(user::query::Query);

#[derive(MergedObject, Default)]
pub struct Mutation(user::mutation::Mutation);
pub struct Subscription;