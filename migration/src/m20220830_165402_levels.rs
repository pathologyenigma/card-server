use sea_orm_migration::{prelude::*, sea_orm::{Statement, ConnectionTrait}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // I need one constraint on multi-feilds which is not yet supported by sea-orm
        // so I using raw sql here
       let sql = r#"
       CREATE TABLE public.level_setting
       (
           id uuid NOT NULL,
           user_id integer NOT NULL,
           title character varying(120) COLLATE pg_catalog."default" NOT NULL,
           is_numberic_level boolean NOT NULL,
           counts integer,
           levels jsonb NOT NULL,
           tip_for_setting_user text COLLATE pg_catalog."default" NOT NULL,
           CONSTRAINT level_settings_pkey PRIMARY KEY (id),
           CONSTRAINT one_user_could_not_have_two_level_setting_of_the_same_name UNIQUE (user_id, title),
           CONSTRAINT user_id FOREIGN KEY (user_id)
               REFERENCES public.user (id) MATCH SIMPLE
               ON UPDATE CASCADE
               ON DELETE CASCADE
       ) TABLESPACE pg_default;"#;

    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // cause we don't have a enum for table
        // so drop table should also be raw sql
        let sql = r#"
        DROP TABLE public.level_setting;
        "#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
