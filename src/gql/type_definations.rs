use async_graphql::MergedObject;

mod user;
pub use user::types as user_types;
#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(user::UserMutation);
