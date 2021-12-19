use async_graphql::EmptySubscription;
use error_handling::BadInputErrorHandler;
use lazy_static::lazy_static;
use regex::Regex;
mod gql;
pub use gql::*;
pub mod entity;
pub use entity::*;
use sea_orm::DbConn;
pub fn build(pool: DbConn) -> Schema {
    let bad_input_error_handler = BadInputErrorHandler::default();
    Schema::build(Query::default(),Mutation::default(), EmptySubscription).data(pool).data(bad_input_error_handler).finish()
}

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;
mod error_handling;
pub use error_handling::Handle as ErrorHandler;
lazy_static! {
    pub static ref EMAIL_VERIFICATION: Regex = Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    pub static ref USERNAME_VERIFICATION: Regex = Regex::new(r"^[\u4E00-\u9FA5A-Za-z0-9]{6,24}$").unwrap();
    pub static ref PASSWORD_VERIFICATION: Regex = Regex::new(r"^[a-zA-Z]\w{8,16}$").unwrap();
}