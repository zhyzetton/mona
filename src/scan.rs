use std::collections::HashSet;
use crate::database::repository;
use crate::media::model::{Media, MediaType};
use crate::{config, database};
use image::GenericImageView;
use image::imageops::FilterType;
use nom_exif::{TrackInfoTag, read_track};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const VIDEO_EXTS: &[&str] = &["mp4", "mkv", "avi", "mov", "flv", "wmv", "ts"];
const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "webp"];

pub fn scan_videos(root: &Path) -> HashSet<PathBuf> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .filter(|p| {
            p.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| VIDEO_EXTS.contains(&ext.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .collect()
}

pub fn find_poster(video_path: &Path) -> Option<PathBuf> {
    let stem = video_path.file_stem()?;
    let stem = stem.to_string_lossy();
    for ext in IMAGE_EXTS {
        let candidate = video_path.with_file_name(format!("{stem}.{ext}"));
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn hash_path(path: &Path) -> String {
    let mut hasher = Sha256::new();
    hasher.update(path.to_string_lossy().as_bytes());
    format!("{:x}", hasher.finalize())
}

// 以源路径 hash 为文件名
pub fn make_thumbnail(src: &Path) -> Result<PathBuf, image::ImageError> {
    let hash = hash_path(src);
    let dest_dir = config::posters_dir();
    std::fs::create_dir_all(&dest_dir)?;
    let dest = dest_dir.join(format!("{}.jpg", hash));
    if dest.exists() {
        return Ok(dest);
    }
    let img = image::open(src)?;
    let (w, h) = img.dimensions();
    let scale = 300.0 / h as f32;
    let new_w = (w as f32 * scale).round() as u32;
    let scaled = img.resize(new_w, 300, FilterType::Lanczos3);
    scaled.save(&dest)?;
    Ok(dest)
}

pub async fn scan_job(dirs: Vec<PathBuf>) -> i64 {
    let mut conn = match database::connection::open() {
        Ok(c) => c,
        Err(e) => {
            println!("打开数据库失败：{e}");
            return 0;
        }
    };
    let _ = database::init(&conn);
    let existing_paths: Vec<String> = repository::get_all_paths(&conn)
        .unwrap_or_default()
        .into_iter()
        .collect();
    let all_video_paths: Vec<PathBuf> =
        dirs.into_iter().flat_map(|dir| scan_videos(&dir)).collect();
    if all_video_paths.is_empty() {
        return 0;
    }
    // 并行处理每个视频
    let results: Vec<Media> = all_video_paths
        .par_iter()
        .filter_map(|path| {
            let file_path = path.to_string_lossy().to_string();
            if existing_paths.contains(&file_path) {
                return None;
            }
            let title = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .replace('_', " ");
            let duration = get_duration(path);
            let poster_path = if let Some(src) = find_poster(path) {
                match make_thumbnail(&src) {
                    Ok(dest) => Some(dest.to_string_lossy().to_string()),
                    Err(e) => {
                        eprintln!("生成缩略图失败 {}: {e}", src.display());
                        None
                    }
                }
            } else {
                None
            };
            let media = Media {
                id: None,
                title,
                year: None,
                overview: None,
                media_type: MediaType::Personal,
                duration,
                rating: None,
                actors: vec![],
                poster_path,
                file_path,
            };
            Some(media)
        })
        .collect();
    if results.is_empty() {
        return 0;
    }
    let tx = match conn.transaction() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("开启事务失败");
            return 0;
        }
    };
    let mut added = 0;
    for media in results {
        if repository::insert_media_with_tx(&tx, &media).is_ok() {
            added += 1;
        } else {
            eprintln!("插入失败");
        }
    }
    if let Err(e) = tx.commit() {
        eprintln!("提交事务失败: {e}");
        return 0;
    }
    added
}

fn get_duration(path: &Path) -> Option<String> {
    let track_info = match read_track(path) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("读取文件元数据失败 {}: {}", path.display(), e);
            return None;
        }
    };
    let duration_ms: Option<u64> = match track_info.get(TrackInfoTag::DurationMs) {
        None => {
            eprintln!("文件 {} 中未找到时长信息", path.display());
            return None;
        }
        Some(v) => v.as_u64(),
    };

    let seconds = duration_ms.unwrap_or(0) / 1000;
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    Some(format!("{:02}:{:02}:{:02}", h, m, s))
}
