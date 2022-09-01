use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       manager.create_table(
        sea_query::Table::create()
        .table(User::Table)
        .if_not_exists()
        .col(
            ColumnDef::new(User::Id)
            .integer()
            .not_null()
            .auto_increment()
            .primary_key()
        )
        .col(
            ColumnDef::new(User::Username)
            .string()
            .not_null()
            .unique_key()
        )
        .col(
            ColumnDef::new(User::Password)
            .string()
            .not_null()
        )
        .col(
            ColumnDef::new(User::Email)
            .string()
            .null()
            .unique_key()
        )
        .to_owned()
       ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // only able to use in with pgsql
        // if you want it to be able to use in other databases
        // you needed to implements it yourself
        manager.drop_table(
            sea_query::Table::drop()
            .table(User::Table)
            .to_owned()
        ).await
    }
}
#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Password,
    Email
}