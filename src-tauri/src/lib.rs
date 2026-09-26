use crate::config::Config;
use crate::database::model::{item, library, SourceType};
use crate::database::repository::Repository;
use crate::errors::AppError;
use crate::media::scanner::local::LocalScanner;
use crate::media::scanner::Scanner;
use tauri::Manager;

pub mod config;
pub mod database;
pub mod errors;
pub mod media;

#[tauri::command]
async fn get_libraries() -> Result<Vec<library::Model>, AppError> {
    Repository::new(database::connection::open().await?)
        .get_libraries()
        .await
}

/// 某个库的顶层条目(剧集/电影,不含季集)
#[tauri::command]
async fn get_videos(library_id: i64) -> Result<Vec<item::Model>, AppError> {
    Repository::new(database::connection::open().await?)
        .get_root_items(library_id)
        .await
}

#[tauri::command]
async fn get_video_detail(item_id: i64) -> Result<(item::Model, Vec<database::model::file::Model>), AppError> {
    Repository::new(database::connection::open().await?)
        .get_item_with_files(item_id)
        .await
}

#[tauri::command]
async fn scan_videos() -> Result<i32, AppError> {
    let repo = Repository::new(database::connection::open().await?);
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

/// 播放:挑一个可用文件交给系统播放器(本地文件路径 = 库根目录 + 相对路径)
#[tauri::command]
async fn play_video(item_id: i64) -> Result<(), AppError> {
    let repo = Repository::new(database::connection::open().await?);
    let item = repo.get_item_by_id(item_id).await?;
    let file = repo.resolve_playable_file(item_id).await?.ok_or_else(|| {
        AppError::FileOperation(format!("这个条目没有可播放的文件: {}", item.title))
    })?;
    let library = repo.get_library_by_id(item.library_id).await?;

    let full_path = std::path::Path::new(&library.root_path).join(&file.relative_path);
    media::player::open_with_system_default(&full_path.to_string_lossy())
        .map_err(|e| AppError::FileOperation(format!("打开播放器失败: {e}")))?;
    Ok(())
}

#[tauri::command]
async fn get_recent_played() -> Result<Vec<item::Model>, AppError> {
    Repository::new(database::connection::open().await?)
        .get_recent_items(5)
        .await
}

/// 开始播放时记一次;播放中更新进度用 update_position
#[tauri::command]
async fn record_play(file_id: i64, position: i64) -> Result<(), AppError> {
    Repository::new(database::connection::open().await?)
        .record_play(file_id, position)
        .await
}

#[tauri::command]
async fn update_position(file_id: i64, position: i64) -> Result<(), AppError> {
    Repository::new(database::connection::open().await?)
        .update_position(file_id, position)
        .await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_libraries,
            get_videos,
            get_video_detail,
            scan_videos,
            get_config,
            save_config,
            play_video,
            get_recent_played,
            record_play,
            update_position
        ])
        .setup(|app| {
            // 先把迁移跑完,再把本地库的根目录加进 asset protocol 白名单
            let local_roots = tauri::async_runtime::block_on(async {
                let db = database::connection::open().await?;
                database::init(&db).await?;
                let roots = Repository::new(db)
                    .get_enabled_libraries()
                    .await?
                    .into_iter()
                    .filter(|library| library.source == SourceType::Local)
                    .map(|library| library.root_path)
                    .collect::<Vec<_>>();
                Ok::<_, AppError>(roots)
            })?;

            let scope = app.asset_protocol_scope();
            for dir in local_roots {
                let _ = scope.allow_directory(&dir, true);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
