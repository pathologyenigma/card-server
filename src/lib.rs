use async_graphql::EmptySubscription;
use lazy_static::lazy_static;
use regex::Regex;
mod gql;
pub use gql::*;
pub mod entity;
pub use entity::*;
use sea_orm::DbConn;
pub fn build(pool: DbConn) -> Schema {
    Schema::build(Query::default(),Mutation::default(), EmptySubscription).data(pool).finish()
}

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

lazy_static! {
    pub static ref EMAIL_VERIFICATION: Regex = Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    pub static ref USERNAME_VERIFICATION: Regex = Regex::new(r"^[\u4E00-\u9FA5A-Za-z0-9]{6,24}$").unwrap();
    pub static ref PASSWORD_VERIFICATION: Regex = Regex::new(r"^[a-zA-Z]\w{8,16}$").unwrap();
}