use async_graphql::EmptySubscription;

mod gql;
pub use gql::*;

pub fn build() -> Schema {
    Schema::new(Query::default(),Mutation::default(), EmptySubscription)
}

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;