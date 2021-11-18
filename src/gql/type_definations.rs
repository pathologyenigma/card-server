use async_graphql::MergedObject;

mod user;

#[derive(MergedObject, Default)]
pub struct Query(user::UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(user::UserMutation);