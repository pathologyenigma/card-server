use async_graphql::EmptySubscription;

mod gql;
pub use gql::*;
pub mod entity;
pub use entity::*;
use sea_orm::DbConn;
pub fn build(pool: DbConn) -> Schema {
    Schema::build(Query::default(),Mutation::default(), EmptySubscription).data(pool).finish()
}

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;