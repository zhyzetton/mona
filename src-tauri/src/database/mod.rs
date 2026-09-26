pub mod connection;
pub mod migration;
pub mod model;
pub mod repository;

use crate::errors::AppError;
use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::MigratorTrait;

pub use migration::Migrator;

/// 打开连接后调用:执行所有未跑的迁移
pub async fn init(db: &DatabaseConnection) -> Result<(), AppError> {
    Migrator::up(db, None).await?;
    Ok(())
}
