use rusqlite::Connection;
use crate::config::db_path;

pub fn open() -> rusqlite::Result<Connection> {
    let path = db_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).unwrap();
    }
    Connection::open(path)
}