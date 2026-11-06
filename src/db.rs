use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn init(db_path: &str) -> Result<Self, sqlx::Error> {
        let db_url = format!("sqlite:{}?mode=rwc", db_path);
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;
        Self::create_tables(&pool).await?;
        Ok(Self { pool })
    }

    async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            -- 1. 媒体库管理表
            CREATE TABLE IF NOT EXISTS libraries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                storage_type TEXT NOT NULL,
                path TEXT NOT NULL,
                webdav_username TEXT,
                webdav_password TEXT,
                scrape BOOLEAN NOT NULL DEFAULT 1,
                created_at INTEGER NOT NULL
            );

            -- 2. 物理文件表
            CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                library_id INTEGER NOT NULL,
                path TEXT NOT NULL,
                storage_type TEXT NOT NULL,
                size INTEGER NOT NULL,
                duration INTEGER,
                resolution TEXT,
                format TEXT NOT NULL,
                FOREIGN KEY (library_id) REFERENCES libraries(id) ON DELETE CASCADE
            );

            -- 3. 统一媒体元数据表（支持电影、剧集、季、单集、本地视频等树状结构）
            CREATE TABLE IF NOT EXISTS media_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                library_id INTEGER NOT NULL,
                parent_id INTEGER,
                file_id INTEGER,
                media_type TEXT NOT NULL,
                title TEXT NOT NULL,
                original_title TEXT,
                year INTEGER,
                overview TEXT,
                poster_path TEXT,
                backdrop_path TEXT,
                rating REAL,
                season_number INTEGER,
                episode_number INTEGER,
                FOREIGN KEY (library_id) REFERENCES libraries(id) ON DELETE CASCADE,
                FOREIGN KEY (parent_id) REFERENCES media_items(id) ON DELETE CASCADE,
                FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE SET NULL
            );
            "#,
        )
            .execute(pool)
            .await?;
        Ok(()).map_err(|e| e)
    }
}