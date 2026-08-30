use crate::database::repository;
use crate::media::model::{Media, MediaType};
use crate::{config, database};
use image::imageops::FilterType;
use image::GenericImageView;
use nom_exif::{read_track, TrackInfoTag};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const VIDEO_EXTS: &[&str] = &["mp4", "mkv", "avi", "mov", "flv", "wmv", "ts"];
const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "webp"];

struct MetaInfo {
    pub duration: String,
    pub resolution: i32,
    pub file_size: String
}

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
pub fn make_thumbnail(src: &Path) -> Result<(PathBuf, PathBuf), image::ImageError> {
    let hash = hash_path(src);

    let poster_dir = config::posters_dir();
    let detail_img_dir = config::detail_img_dir();
    std::fs::create_dir_all(&poster_dir)?;
    std::fs::create_dir_all(&detail_img_dir)?;

    let poster_dest = poster_dir.join(format!("{}.jpg", hash));
    let detail_ext = src.extension().and_then(|e| e.to_str()).unwrap_or("jpg");
    let detail_dest = detail_img_dir.join(format!("{}.{}", hash, detail_ext));
    if poster_dest.exists() && detail_dest.exists() {
        return Ok((poster_dest, detail_dest));
    }
    let img = image::open(src)?;
    let (w, h) = img.dimensions();
    let scale = 300.0 / h as f32;
    let new_w = (w as f32 * scale).round() as u32;
    let scaled = img.resize(new_w, 300, FilterType::Lanczos3);
    scaled.save(&poster_dest)?;
    fs::copy(src, &detail_dest)?;
    Ok((poster_dest, detail_dest))
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;        // 1,048,576
    const GB: u64 = MB * 1024;        // 1,073,741,824

    if bytes <= GB {
        // 不超过 1G → 显示 MB
        let mb = bytes as f64 / MB as f64;
        format!("{:.2} MB", mb)
    } else {
        // 超过 1G → 显示 GB
        let gb = bytes as f64 / GB as f64;
        format!("{:.2} GB", gb)
    }
}

pub async fn scan_job(dirs: Vec<PathBuf>) -> i64 {
    let result = tokio::task::spawn_blocking(move || {
        // 1. 打开数据库（同步阻塞，但现在在阻塞线程池中执行）
        let mut conn = match database::connection::open() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("打开数据库失败：{e}");
                return 0;
            }
        };

        // 2. 获取已存在的路径，使用 HashSet 加速查找
        let existing_paths: HashSet<String> = repository::get_all_paths(&conn)
            .unwrap_or_default()
            .into_iter()
            .collect();

        // 3. 扫描所有视频路径（同样在阻塞线程中）
        let all_video_paths: Vec<PathBuf> =
            dirs.into_iter().flat_map(|dir| scan_videos(&dir)).collect();

        if all_video_paths.is_empty() {
            return 0;
        }

        // 4. Rayon 并行处理
        let results: Vec<Media> = all_video_paths
            .par_iter()
            .filter_map(|path| {
                let file_path = path.to_string_lossy().to_string();
                // 现在 existing_paths 是 HashSet，查找 O(1)
                if existing_paths.contains(&file_path) {
                    return None;
                }
                let title = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .replace('_', " ");
                // 获取元数据失败就跳过
                let metainfo = match get_metainfo(path) {
                    Some(m) => m,
                    None => return None
                };
                let duration = metainfo.duration;
                let file_size = metainfo.file_size;

                let (poster_path, detail_path) = if let Some(src) = find_poster(path) {
                    match make_thumbnail(&src) {
                        Ok((poster_dest, detail_dest)) => (
                            Some(poster_dest.to_string_lossy().to_string()),
                            Some(detail_dest.to_string_lossy().to_string()),
                        ),
                        Err(e) => {
                            eprintln!("生成缩略图失败 {}: {e}", src.display());
                            (None, None)
                        }
                    }
                } else {
                    (None, None)
                };

                Some(Media {
                    id: None,
                    title,
                    year: None,
                    overview: None,
                    media_type: MediaType::Local,
                    duration,
                    rating: None,
                    actors: vec![],
                    poster_path,
                    detail_img_path: detail_path,
                    file_path,
                    file_size,
                    resolution: metainfo.resolution
                })
            })
            .collect();

        if results.is_empty() {
            return 0;
        }

        // 5. 事务插入（仍在阻塞线程中）
        let tx = match conn.transaction() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("开启事务失败: {e}");
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
    })
    .await; // 等待阻塞任务完成

    // 处理可能的 panic 或取消
    result.unwrap_or_else(|e| {
        eprintln!("扫描任务 panic 或取消: {e}");
        0
    })
}

fn get_metainfo(path: &Path) -> Option<MetaInfo> {
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
    let duration = format!("{:02}:{:02}:{:02}", h, m, s);

    let h: Option<u32> = match track_info.get(TrackInfoTag::Height) {
        None => {
            eprintln!("文件 {} 中未找到高度信息", path.display());
            None
        }
        Some(v) => v.as_u32()
    };
    let file_size_byte = fs::metadata(path).ok().map(|m| m.len());
    let file_size = format_bytes(file_size_byte.unwrap_or(0_u64));
    Some(MetaInfo {
        duration,
        resolution: h.unwrap_or(0_u32) as i32,
        file_size
    })
}

