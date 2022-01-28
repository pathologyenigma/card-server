#[macro_use]
extern crate tantivy;
use async_graphql::EmptySubscription;
use error_handling::BadInputErrorHandler;
use lazy_static::lazy_static;
use regex::Regex;
mod gql;
pub use gql::*;
pub mod entity;
pub use entity::*;
use sea_orm::DbConn;
mod text_search;
pub fn build(pool: DbConn) -> Schema {
    let bad_input_error_handler = BadInputErrorHandler::default();
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(pool)
        .data(bad_input_error_handler)
        .finish()
}
mod tokenizer;
pub use tokenizer::Token;
pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;
mod error_handling;
pub use error_handling::*;
lazy_static! {
    pub static ref EMAIL_VERIFICATION: Regex =
        Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    pub static ref USERNAME_VERIFICATION: Regex =
        Regex::new(r"^[\u4E00-\u9FA5A-Za-z0-9]{6,24}$").unwrap();
    pub static ref PASSWORD_VERIFICATION: Regex = Regex::new(r"^[a-zA-Z]\w{8,16}$").unwrap();
}
#[derive(Clone, Debug)]
pub struct TokenFromHeader(pub String);
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use cang_jie::CANG_JIE;
    use cang_jie::CangJieTokenizer;
    use cang_jie::TokenizerOption;
    use jieba_rs::Jieba;
    use tantivy::Index;
    use tantivy::IndexSettings;
    use tantivy::schema::*;
    use tantivy::directory::MmapDirectory;
    #[test]
    fn text_search() -> tantivy::Result<()> {
        let mut schema_builder = SchemaBuilder::default();
        let text_indexing = TextFieldIndexing::default()
            .set_tokenizer(CANG_JIE)
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_indexing)
            .set_stored();
        schema_builder.add_u64_field("id", STORED);
        schema_builder.add_text_field("name", text_options.clone());
        schema_builder.add_text_field("description", text_options);
        schema_builder.add_text_field("logo", STORED);
        let schema = schema_builder.build();
        let db = std::fs::DirBuilder::new();
        if !std::path::Path::new("tantivy").exists() {
            db.create("tantivy").unwrap();
        }
        if !std::path::Path::new("tantivy/cards").exists() {
            db.create("tantivy/cards").unwrap();
        }
        let path = MmapDirectory::open("tantivy/cards")?;
        let index = Index::create(path, schema, IndexSettings::default())?;
        let tokenizer = CangJieTokenizer {
            worker: Arc::new(Jieba::empty()),
            option: TokenizerOption::Unicode,
        };
        index.tokenizers().register(CANG_JIE, tokenizer);
        Ok(())
    }
}
