use rusqlite::{params, Connection, Result};
use crate::media::model::{Media, MediaType};

pub fn insert_media(conn: &Connection, media: &Media) -> Result<()> {
    conn.execute(
        "\
        INSERT INTO media (\
        title, \
        year,\
        overview,\
        media_type,\
        duration, \
        rating,\
        actors,\
        poster_path,\
        file_path\
        )\
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &media.title,
            media.year.as_deref().unwrap_or(""),
            &media.overview,
            media.media_type.to_i64(),
            &media.duration,
            media.rating,
            serde_json::to_string(&media.actors).unwrap(),
            &media.poster_path,
            &media.file_path,
            )
    )?;
    Ok(())
}

pub fn find_all(conn: &Connection) -> Result<Vec<Media>> {
    let mut stmt = conn.prepare("SELECT \
    id,\
    title,\
    year,\
    overview,\
    media_type,\
    duration,
    rating,\
    actors,\
    poster_path,\
    file_path \
    FROM media")?;
    let rows = stmt.query_map([], |row| {
        let media_type_value: i64 = row.get(4)?;
        let media_type = MediaType::from_i64(media_type_value).ok_or_else(|| {
            rusqlite::Error::InvalidColumnType(
                4,
                "media_type".to_string(),
                rusqlite::types::Type::Integer,
            )
        })?;
        let actors_json: String = row.get(7)?;
        let actors: Vec<String> = serde_json::from_str(&actors_json).map_err(|_| {
            rusqlite::Error::InvalidQuery
        })?;

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
            file_path: row.get(9)?,
        })
    })?;
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