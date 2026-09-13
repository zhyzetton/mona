use crate::errors::AppError;
use crate::media::model::{Media, MediaType};
use rusqlite::{params, Connection, Result, Transaction};
use std::sync::{Arc, Mutex};

const MEDIA_INSERT_COL_NAMES: &str = "title, year, overview, media_type,\
    duration, rating, actors, poster_path, detail_img_path,\
    file_path, file_size, resolution, source, tags, added_at";

const RECORD_COL_NAMES: &str = "media_id, played_at, play_count";

pub struct Repository {
    conn: Arc<Mutex<Connection>>,
}

impl Repository {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }

    pub fn with_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(Transaction<'_>) -> Result<T>,
    {
        let mut guard = self.conn.lock().unwrap();
        let tx = guard.transaction()?;
        let result = f(tx)?;
        Ok(result)
    }

    pub fn get_media_by_id(&self, id: i64) -> Result<Media, AppError> {
        let conn = self.conn();
        let mut stmt = conn
            .prepare(&format!(
                "SELECT id, {MEDIA_INSERT_COL_NAMES} FROM media WHERE id = ?1"
            ))
            .map_err(|_| AppError::Database("构建查询语句失败".to_string()))?;

        stmt.query_row([id], row_to_media)
            .map_err(|_| AppError::Database("查询失败".to_string()))
    }

    pub fn get_recent_media(&self, limit: i64) -> Result<Vec<Media>, AppError> {
        let conn = self.conn();
        let sql = format!(
            "SELECT id, {MEDIA_INSERT_COL_NAMES} FROM media m \
             JOIN play_history h ON h.media_id = m.id \
             ORDER BY h.played_at DESC LIMIT ?1"
        );
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|_| AppError::Database("构建查询语句失败".to_string()))?;
        let rows = stmt
            .query_map(params![limit], row_to_media)
            .map_err(|_| AppError::Database("查询失败".to_string()))?
            .collect::<Result<Vec<Media>>>()
            .map_err(|_| AppError::Database("读取查询结果失败".to_string()))?;
        Ok(rows)
    }

    // 记录一次播放:条目已有记录则刷新播放时间并累加次数,否则插入新行
    pub fn record_play(&self, media_id: i64) -> Result<()> {
        let conn = self.conn();
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM media WHERE id = ?1",
            [media_id],
            |row| row.get(0),
        )?;
        if exists == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        conn.execute(
            &format!(
                "INSERT INTO play_history ({RECORD_COL_NAMES}) VALUES (?1, ?2, ?3) \
                 ON CONFLICT(media_id) DO UPDATE SET \
                 played_at = excluded.played_at, play_count = play_count + 1"
            ),
            params![media_id, now, 1],
        )?;
        Ok(())
    }

    pub fn find_all(&self) -> Result<Vec<Media>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&format!("SELECT id, {MEDIA_INSERT_COL_NAMES} FROM media"))?;
        let rows = stmt.query_map([], row_to_media)?;
        rows.collect()
    }

    pub fn exists_by_path(&self, path: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM media WHERE file_path = ?1")?;
        let count: i64 = stmt.query_row(params![path], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn update_poster(&self, id: i64, poster_path: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE media SET poster_path = ?1 WHERE id = ?2",
            params![poster_path, id],
        )?;
        Ok(())
    }

    pub fn get_all_paths(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT file_path FROM media")?;
        let paths = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>>>()?;
        Ok(paths)
    }

    // 单条插入 SQL(关联函数):单条/批量共用,事务由调用方提供
    fn insert_media_tx(tx: &Transaction<'_>, media: &Media) -> Result<i64> {
        let media_type = media.media_type.unwrap_or(MediaType::Personal);
        tx.execute(
            &format!("INSERT INTO media ({MEDIA_INSERT_COL_NAMES}) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)"),
            (
                &media.title,
                &media.year,
                &media.overview,
                media_type.to_i64(),
                &media.duration,
                &media.rating,
                serde_json::to_string(&media.actors).unwrap(),
                &media.poster_path,
                &media.detail_img_path,
                &media.file_path,
                &media.file_size,
                &media.resolution,
                &media.source.to_i64(),
                serde_json::to_string(&media.tags).unwrap(),
                &media.added_at,
            ),
        )?;
        Ok(tx.last_insert_rowid())
    }

    pub fn insert_media(&self, media: &Media) -> Result<i64, AppError> {
        self.with_transaction(|tx| -> Result<i64> {
            let last_id = Self::insert_media_tx(&tx, media)?;
            tx.commit()?;
            Ok(last_id)
        })
        .map_err(|e| AppError::Database(format!("事务执行失败: {e}")))
    }
    
    pub fn insert_media_batch(&self, items: &[Media]) -> Result<usize, AppError> {
        self.with_transaction(|tx| -> Result<usize> {
            let mut count = 0;
            for media in items {
                Self::insert_media_tx(&tx, media)?;
                count += 1;
            }
            tx.commit()?;
            Ok(count)
        })
        .map_err(|e| AppError::Database(format!("事务执行失败: {e}")))
    }
}

fn row_to_media(row: &rusqlite::Row) -> Result<Media> {
    let media_type_value: i64 = row.get(4)?;
    let media_type = MediaType::from_i64(media_type_value);
    let actors_json: String = row.get(7)?;
    let actors: Vec<String> =
        serde_json::from_str(&actors_json).map_err(|_| rusqlite::Error::InvalidQuery)?;

    let source_value: i64 = row.get(13)?;
    let source = crate::media::model::SourceType::from_i64(source_value)
        .ok_or_else(|| rusqlite::Error::InvalidQuery)?;
    let tags_json: String = row.get(14)?;
    let tags: Vec<String> =
        serde_json::from_str(&tags_json).map_err(|_| rusqlite::Error::InvalidQuery)?;
    let added_at = row.get(15)?;

    Ok(Media {
        id: row.get(0)?,
        title: row.get(1)?,
        year: row.get(2)?,
        overview: row.get(3)?,
        media_type,
        duration: row.get(5)?,
        rating: row.get(6)?,
        actors,
        poster_path: row.get(8)?,
        detail_img_path: row.get(9)?,
        file_path: row.get(10)?,
        file_size: row.get(11)?,
        resolution: row.get(12)?,
        source,
        tags,
        added_at,
    })
}
