use error_handling::BadInputErrorHandler;
use lazy_static::lazy_static;
use regex::Regex;
mod gql;
pub use gql::*;
pub mod entity;
pub use entity::*;
use redis::Client;
use sea_orm::DbConn;
// fn init_tantivy(ctx: Schema) -> Schema {
    
// }
pub fn build(pool: DbConn) -> Schema {
    let bad_input_error_handler = BadInputErrorHandler::default();
    let client = Client::open("redis://127.0.0.1/").unwrap();
    Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(pool)
    .data(bad_input_error_handler)
    .data(client)
    .finish()
}
mod tokenizer;
pub use tokenizer::{on_connection_init, Token};
pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;
mod error_handling;
pub use error_handling::*;
lazy_static! {
    pub static ref EMAIL_VERIFICATION: Regex =
        Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    pub static ref USERNAME_VERIFICATION: Regex =
        Regex::new(r"^[\u4E00-\u9FA5A-Za-z0-9]{6,24}$").unwrap();
    pub static ref PASSWORD_VERIFICATION: Regex = Regex::new(r"^[a-zA-Z]{8,16}$").unwrap();
}
#[derive(Clone, Debug)]
pub struct TokenFromHeader(pub String);

