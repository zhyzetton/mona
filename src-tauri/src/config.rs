use dirs;
use serde;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct Config {
    #[serde(default)]
    pub local_dirs: Vec<PathBuf>,
    pub player_name: Option<String>,
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

    pub fn save(&self) -> Result<(), String> {
        let path = config_file_path().unwrap();
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).unwrap()
        }
        let content = toml::to_string_pretty(self).unwrap();
        fs::write(path, content).map_err(|e| e.to_string())
    }
}

pub fn posters_dir() -> PathBuf {
    dirs::home_dir()
        .map(|home| home.join(".mona").join("posters"))
        .unwrap_or_default()
}

pub fn detail_img_dir() -> PathBuf {
    dirs::home_dir()
        .map(|home| home.join(".mona").join("detail_imgs"))
        .unwrap_or_default()
}

pub fn db_path() -> PathBuf {
    dirs::home_dir()
        .map(|home| home.join(".mona").join("library.db"))
        .unwrap()
}

fn config_file_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".mona").join("config.toml"))
}
