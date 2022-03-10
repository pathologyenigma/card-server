#[macro_use]
extern crate tantivy;
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
#[cfg(test)]
mod tests {
    use cang_jie::CangJieTokenizer;
    use cang_jie::TokenizerOption;
    use cang_jie::CANG_JIE;
    use jieba_rs::Jieba;
    use tantivy::collector::TopDocs;
    use std::sync::Arc;
    use tantivy::directory::MmapDirectory;
    use tantivy::schema::*;
    use tantivy::Index;
    use tantivy::IndexSettings;
    use tantivy::query::QueryParser;
    #[test]
    fn text_search() -> tantivy::Result<()> {
        let mut schema_builder = SchemaBuilder::default();
        let text_indexing = TextFieldIndexing::default()
            .set_tokenizer(CANG_JIE)
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_indexing)
            .set_stored();
        let id = schema_builder.add_u64_field("id", STORED | INDEXED);
        let name = schema_builder.add_text_field("name", text_options.clone());
        let description = schema_builder.add_text_field("description", text_options);
        let logo = schema_builder.add_text_field("logo", STORED);
        let schema = schema_builder.build();
        let db = std::fs::DirBuilder::new();
        if !std::path::Path::new("tantivy").exists() {
            db.create("tantivy").unwrap();
        }
        if !std::path::Path::new("tantivy/cards").exists() {
            db.create("tantivy/cards").unwrap();
        }
        let path = MmapDirectory::open("tantivy/cards")?;
        let index = Index::create(path, schema.clone(), IndexSettings::default())?;
        let tokenizer = CangJieTokenizer {
            worker: Arc::new(Jieba::empty()),
            option: TokenizerOption::Unicode,
        };
        index.tokenizers().register(CANG_JIE, tokenizer);
        let mut writer = index.writer(100_000_000)?;
        writer.add_document(doc!(
            id => 1u64,
            name => "pathology",
            description => "a brutal slamming death metal band which is my favorite",
            logo => "https://www.nocleansinging.com/wp-content/uploads/2013/09/Pathology-Lords-of-Rephaim.jpg",
        ));
        writer.commit().expect("failed to commit document insertation");
        let reader = index.reader().expect("failed to get reader");
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![name, description]);
        let query = query_parser.parse_query("pathology metal")?;
        let res = searcher.search(&query, &TopDocs::with_limit(10))?;
        for (score, doc_address) in res {
            let doc = searcher.doc(doc_address)?;
            println!("found doc: \n{{\n\tscore: {}, \n\tdoc: {}\n}}", score, schema.to_json(&doc));
        }
        Ok(())
    }
}
