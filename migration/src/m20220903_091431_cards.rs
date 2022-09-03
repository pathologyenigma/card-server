use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectionTrait, Statement},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // same reason with the level_settings
        let sql = r#"
        CREATE TABLE public.card
        (
            id uuid NOT NULL,
            name character varying NOT NULL,
            description character varying NOT NULL,
            logo character varying,
            effects text,
            created_by integer NOT NULL,
            PRIMARY KEY (id),
            CONSTRAINT one_card_name_could_only_exist_one_for_one_user UNIQUE (name, created_by),
            CONSTRAINT created_by FOREIGN KEY (created_by)
                REFERENCES public."user" (id) MATCH SIMPLE
                ON UPDATE NO ACTION
                ON DELETE NO ACTION
                NOT VALID
        );"#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // so I using raw sql here
        let sql = r#"
        DROP TABLE public.card;
        "#;

        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
