use crate::config::db_path;
use crate::errors::AppError;
use sea_orm::sqlx::sqlite::{SqliteJournalMode, SqliteSynchronous};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn open() -> Result<DatabaseConnection, AppError> {
    let path = db_path().ok_or_else(|| AppError::FileOperation("数据库路径打开失败".to_string()))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| AppError::FileOperation(e.to_string()))?;
    }

    // sqlx 按字符串切分 URL,Windows 路径要转成正斜杠;mode=rwc 表示文件不存在就创建
    let url = format!(
        "sqlite://{}?mode=rwc",
        path.to_string_lossy().replace('\\', "/")
    );
    let mut options = ConnectOptions::new(url);
    options
        .max_connections(5)
        .sqlx_logging(false)
        .map_sqlx_sqlite_opts(|opts| {
            opts.journal_mode(SqliteJournalMode::Wal)
                .busy_timeout(Duration::from_secs(5))
                .synchronous(SqliteSynchronous::Normal)
        });

    Database::connect(options).await.map_err(AppError::from)
}
