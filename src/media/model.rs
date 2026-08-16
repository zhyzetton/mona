pub struct Media {
    pub id: i64,
    pub title: String,
    pub year: Option<i32>,
    pub overview: Option<String>,
    pub media_type: MediaType,
    pub rating: Option<f32>,
    pub actors: Vec<String>,
    pub poster_path: Option<String>,
    pub file_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    Movie,
    Series,
    Anime,
    Personal,
}
