#[derive(Debug, Clone)]
pub struct Media {
    pub id: Option<i64>,
    pub title: String,
    pub year: Option<String>,
    pub overview: Option<String>,
    pub media_type: MediaType,
    pub duration: Option<String>,
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

impl MediaType {
    pub fn to_i64(self) -> i64 {
        match self {
            MediaType::Movie => 0,
            MediaType::Series => 1,
            MediaType::Anime => 2,
            MediaType::Personal => 3,
        }
    }

    pub fn from_i64(value: i64) -> Option<Self> {
        match value {
            0 => Some(Self::Movie),
            1 => Some(Self::Series),
            2 => Some(Self::Anime),
            3 => Some(Self::Personal),
            _ => None,
        }
    }
}
