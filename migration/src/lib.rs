pub use sea_orm_migration::prelude::*;

mod m20220830_164929_users;
mod m20220830_165402_levels;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220830_164929_users::Migration),
            Box::new(m20220830_165402_levels::Migration),
        ]
    }
}
