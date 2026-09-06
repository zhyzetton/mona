use crate::config::Config;
use crate::media::model::Media;
use tauri::{Manager};
use crate::errors::AppError;

pub mod config;
pub mod database;
pub mod media;
pub mod errors;

#[tauri::command]
fn get_videos() -> Result<Vec<Media>, String> {
    let conn = database::connection::open().map_err(|e| e.to_string())?;
    database::repository::find_all(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
async fn scan_videos() -> Result<i32, AppError> {
    let config_map = Config::load();
    let video_path = config_map.local_dirs;
    let result_count = media::scan::scan_job(video_path).await;
    result_count
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
    let conn = database::connection::open()?;

    let video = database::repository::get_media_by_id(&conn, video_id)?;
    let result = media::player::open_with_system_default(&video.file_path);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(AppError::FileOperation("保存失败".to_string())),
    }
}

#[tauri::command]
fn get_recent_played() -> Result<Vec<Media>, AppError> {
    let conn = database::connection::open()?;
    database::repository::get_recent_media(&conn, 5)
}

#[tauri::command]
fn record_play(media_id: i64) -> Result<(), AppError> {
    let conn = database::connection::open()?;
    database::repository::record_play(&conn, media_id)?;
    Ok(())
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
