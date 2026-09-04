use rusqlite::Connection;
use crate::config::db_path;
use crate::errors::AppError;

pub fn open() -> rusqlite::Result<Connection, AppError> {
    let path = db_path().ok_or_else(|| AppError::FileOperation("数据库路径打开失败".to_string()))?; 
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).unwrap();
    }
    Connection::open(path).map_err(|e| AppError::Database("打开数据库连接失败".to_string()))
}