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
            resolution INTEGER,
            source INTEGER,
            tags TEXT,
            added_at INTEGER NOT NULL DEFAULT 0
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_media_file_path ON media(file_path);
            "#,
    )?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS play_history (
            media_id INTEGER PRIMARY KEY,
            played_at INTEGER NOT NULL,
            play_count INTEGER NOT NULL DEFAULT 1
            );
            CREATE INDEX IF NOT EXISTS idx_play_history_played_at ON play_history(played_at);
            "#,
    )?;

    Ok(())
}
