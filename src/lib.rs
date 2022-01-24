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
    Schema::build(Query::default(),Mutation::default(), EmptySubscription).data(pool).data(bad_input_error_handler).finish()
}

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;
mod error_handling;
pub use error_handling::ErrorHandler;
lazy_static! {
    pub static ref EMAIL_VERIFICATION: Regex = Regex::new(r"^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap();
    pub static ref USERNAME_VERIFICATION: Regex = Regex::new(r"^[\u4E00-\u9FA5A-Za-z0-9]{6,24}$").unwrap();
    pub static ref PASSWORD_VERIFICATION: Regex = Regex::new(r"^[a-zA-Z]\w{8,16}$").unwrap();
}

#[cfg(test)]
mod tests {
    use async_graphql::extensions::Extension;
    use tantivy::ReloadPolicy;
    use tantivy::collector::TopDocs;
    use tantivy::query::QueryParser;
    use tantivy::schema::*;
    #[test]
    fn text_search() -> tantivy::Result<()>{
        let index = tantivy::Index::open_in_dir("tantivy/wikipedia-index")?;
        let schema = index.load_metas()?.schema;
        let title = schema.get_field("title").expect("fuck it not works");
        let body = schema.get_field("body").expect("fuck it not works");
        let reader = index.reader_builder().reload_policy(ReloadPolicy::OnCommit).try_into()?;
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![title, body]);
        let query = query_parser.parse_query("ok")?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(20))?;
        let mut counter = 0;
        for (index, (score, doc_address)) in top_docs.into_iter().enumerate() {
            let retrieved_doc = searcher.doc(doc_address)?;
            counter = index;
            println!("score: {}, {}", score, schema.to_json(&retrieved_doc));
        }
        println!("get {} docs in all", counter);
        Ok(())
    }
}