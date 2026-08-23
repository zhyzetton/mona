use std::{fs, io};
use serde;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use dirs;

#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct Config {
    #[serde(default)]
    pub local_dirs: Vec<PathBuf>,
    pub player_name: Option<String>
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

    pub fn save(&self) {
        let Some(path) = config_file_path() else {
            return;
        };
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let content = toml::to_string_pretty(self).unwrap();
        let _ = fs::write(path, content);
    }
}

pub fn posters_dir() -> PathBuf {
    dirs::home_dir().map(|home| home.join(".mona").join("posters"))
        .unwrap_or_default()
}

fn config_file_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".mona").join("config.toml"))
}

