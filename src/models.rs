use sqlx::FromRow;

#[derive(Default, FromRow)]
pub struct File {
    pub id: i64,
    pub library_id: i64,
    pub path: String,
    pub storage_type: StorageType,
    pub size: i64,
    pub duration: Option<i64>,
    pub resolution: Option<String>,
    pub format: FileFormat
}

#[derive(Default, FromRow)]
pub struct MediaItem {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub file_id: Option<i64>,
    pub library_id: i64,
    pub media_type: MediaType,
    pub title: String,
    pub year: Option<i64>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub rating: Option<f64>,
    pub season_number: Option<i64>,
    pub episode_number: Option<i64>,
}

#[derive(Default, FromRow)]
pub struct Library {
    pub id: i64,
    pub name: String,
    pub storage_type: StorageType,
    pub path: String,
    pub webdav_username: Option<String>,
    pub webdav_password: Option<String>,
    pub scrape: bool,
    pub created_at: i64
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum StorageType {
    #[default]
    Local,
    WebDav
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum FileFormat {
    #[default]
    Mp4,
    MKV,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum MediaType {
    #[default]
    Video, // 未索引/本地 视频
    Movie,
    Anime,
    Series,
    Documentary,
    Concert,
    Season,
    Episode
}