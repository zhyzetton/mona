pub mod connection;
pub mod repository;

use rusqlite::Connection;

pub fn init(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS media (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            year INTEGER,
            overview TEXT,
            media_type INTEGER NOT NULL,
            rating REAL,
            actors TEXT,
            poster_path TEXT,
            file_path TEXT NOT NULL
            );
            "#,
    )?;
    Ok(())
}
