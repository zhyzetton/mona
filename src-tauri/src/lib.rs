use crate::config::Config;
use crate::database::repository::Repository;
use crate::media::model::Media;
use crate::media::scanner::local::LocalScanner;
use crate::media::scanner::Scanner;
use tauri::{Manager};
use crate::errors::AppError;

pub mod config;
pub mod database;
pub mod media;
pub mod errors;

#[tauri::command]
fn get_videos() -> Result<Vec<Media>, AppError> {
    let repo = Repository::new(database::connection::open()?);
    repo.find_all().map_err(|e| AppError::Database(format!("获取视频失败: {}", e.to_string())))
}

#[tauri::command]
async fn scan_videos() -> Result<i32, AppError> {
    let repo = Repository::new(database::connection::open()?);
    let added = LocalScanner::new(Config::load().local_dirs)
        .scan(&repo)
        .await?;
    Ok(added.len() as i32)
}

#[tauri::command]
fn get_config() -> Config {
    Config::load()
}

#[tauri::command]
fn save_config(config: Config) -> Result<(), AppError> {
    Config::save(&config)
}

#[tauri::command]
fn play_video(video_id: i64) -> Result<(), AppError> {
    let repo = Repository::new(database::connection::open()?);
    let video = repo.get_media_by_id(video_id)?;
    media::player::open_with_system_default(&video.file_path)
        .map_err(|e| AppError::FileOperation(format!("打开播放器失败: {e}")))?;
    Ok(())
}

#[tauri::command]
fn get_recent_played() -> Result<Vec<Media>, AppError> {
    let repo = Repository::new(database::connection::open()?);
    repo.get_recent_media(5)
}

#[tauri::command]
fn record_play(media_id: i64) -> Result<(), AppError> {
    let repo = Repository::new(database::connection::open()?);
    repo.record_play(media_id)
        .map_err(|e| AppError::Database(e.to_string()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_videos,
            scan_videos,
            get_config,
            save_config,
            play_video,
            get_recent_played,
            record_play
        ])
        .setup(|app| {
            let conn = database::connection::open()?;
            database::init(&conn)?;
            // 将视频目录加入 asset protocol 允许范围
            let scope = app.asset_protocol_scope();
            for dir in Config::load().local_dirs {
                let _ = scope.allow_directory(&dir, true);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
