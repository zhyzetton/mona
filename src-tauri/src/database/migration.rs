pub mod m20260924_000001_init;

use sea_orm_migration::prelude::*;

/// 迁移入口:database::init 会按顺序把没跑过的迁移跑一遍
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260924_000001_init::Migration)]
    }
}
