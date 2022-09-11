use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, Statement},
};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // gin is not enough for my case, if you only wants a gin use the code below
        // manager.create_index(sea_query::Index::create().full_text())
        // here is the text search supported by PGroonga
        let sql = r#"
        SET enable_seqscan = off;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ()).unwrap();
        let sql = r#"
        CREATE INDEX card_search_index ON card USING pgroonga (
            name pgroonga_text_full_text_search_ops_v2,
            description pgroonga_text_regexp_ops_v2,
            effects pgroonga_text_term_search_ops_v2
        );
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        DROP INDEX card_search_index ON card;
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
