use dirs;
use serde;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io};
use crate::errors::AppError;

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct Config {
    #[serde(default)]
    pub local_dirs: Vec<PathBuf>,
    pub player_name: Option<String>,
    pub webdav_info: Option<WebDavInfo>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct WebDavInfo {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn load() -> Self {
        let Some(path) = config_file_path() else {
            return Config::default();
        };
        match fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Config::default(),
            Err(e) => {
                println!("读取配置失败！{e}");
                Config::default()
            }
        }
    }

    pub fn save(&self) -> Result<(), AppError> {
        let path = config_file_path().unwrap();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).unwrap()
        }
        let content = toml::to_string_pretty(self).unwrap();
        fs::write(path, content).map_err(|_| AppError::FileOperation("保存失败".to_string()))
    }
}

pub fn posters_dir() -> Option<PathBuf> {
    dirs::home_dir()
        .map(|home| home.join(".mona").join("posters"))
}

pub fn detail_img_dir() -> Option<PathBuf> {
    dirs::home_dir()
        .map(|home| home.join(".mona").join("detail_imgs"))
}

pub fn db_path() -> Option<PathBuf> {
    dirs::home_dir()
        .map(|home| home.join(".mona").join("library.db"))
}

fn config_file_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".mona").join("config.toml"))
}
