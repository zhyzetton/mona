pub mod connection;
pub mod repository;

use rusqlite::Connection;

pub fn init(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS media (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            year TEXT,
            overview TEXT,
            media_type INTEGER NOT NULL,
            duration TEXT,
            rating REAL,
            actors TEXT,
            poster_path TEXT,
            detail_img_path TEXT,
            file_path TEXT NOT NULL,
            file_size TEXT NOT NULL,
            resolution u64
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_media_file_path ON media(file_path);
            "#,
    )?;
    Ok(())
}
