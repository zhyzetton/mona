use crate::errors::AppError;
use crate::media::model::{Media, MediaType};
use rusqlite::{params, Connection, Result, Transaction};

const MEDIA_COL_NAMES: &str = "id, title, year, overview, media_type,\
    duration, rating, actors, poster_path, detail_img_path,\
    file_path, file_size, resolution, source, tags, added_at";

// INSERT 用列名:不含 id(自增主键),从 title 开始
const MEDIA_INSERT_COL_NAMES: &str = "title, year, overview, media_type,\
    duration, rating, actors, poster_path, detail_img_path,\
    file_path, file_size, resolution, source, tags, added_at";

const RECORD_COL_NAMES: &str = "media_id, played_at, play_count";

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

pub fn get_media_by_id(conn: &Connection, video_id: i64) -> Result<Media, AppError> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {MEDIA_COL_NAMES} FROM media WHERE id = ?1"
        ))
        .map_err(|_| AppError::Database("构建查询语句失败".to_string()))?;

    stmt.query_row([video_id], row_to_media)
        .map_err(|_| AppError::Database("查询失败".to_string()))
}

pub fn get_recent_media(conn: &Connection, limit: i64) -> Result<Vec<Media>, AppError> {
    let sql = format!(
        "SELECT {MEDIA_COL_NAMES} FROM media m \
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
pub fn record_play(conn: &Connection, media_id: i64) -> Result<()> {
    // 没有物理外键,手动校验条目存在,避免已删除的媒体在历史里留孤儿记录
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

pub fn find_all(conn: &Connection) -> Result<Vec<Media>> {
    let mut stmt = conn.prepare(&format!("SELECT {MEDIA_COL_NAMES} FROM media"))?;
    let rows = stmt.query_map([], row_to_media)?;
    rows.collect()
}

pub fn exists_by_path(conn: &Connection, path: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM media WHERE file_path = ?1")?;
    let count: i64 = stmt.query_row(params![path], |row| row.get(0))?;
    Ok(count > 0)
}

pub fn update_poster(conn: &Connection, id: i64, poster_path: &str) -> Result<()> {
    conn.execute(
        "UPDATE media SET poster_path = ?1 WHERE id = ?2",
        params![poster_path, id],
    )?;
    Ok(())
}

pub fn get_all_paths(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT file_path FROM media")?;
    let paths = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;
    Ok(paths)
}

// 事务插入
pub fn insert_media_with_tx(tx: &Transaction, media: &Media) -> Result<i64> {
    let media_type = media.media_type.unwrap_or(MediaType::Personal);
    tx.execute(
        &format!("INSERT INTO media ({MEDIA_INSERT_COL_NAMES}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)"),
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
            &media.added_at
        ),
    )?;
    Ok(tx.last_insert_rowid())
}
