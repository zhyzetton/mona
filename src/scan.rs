use std::fs;
use std::path::{Path, PathBuf};
use image::GenericImageView;
use image::imageops::FilterType;
use walkdir::WalkDir;
use crate::{config, database, scan};
use crate::app::AppState;
use crate::database::repository;
use crate::media::model::{Media, MediaType};

const VIDEO_EXTS: &[&str] = &["mp4", "mkv", "avi", "mov", "flv", "wmv", "ts"];
const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "webp"];

pub fn scan_videos(root: &Path) -> Vec<PathBuf> {
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
        }).collect()
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

pub fn make_thumbnail(src: &Path, dest: &Path) -> Result<(), image::ImageError> {
    let img = image::open(src)?;
    let (w, h) = img.dimensions();
    let scale = 300.0 / h as f32;
    let new_w = (w as f32 * scale).round() as u32;
    let scaled = img.resize(new_w, 300, FilterType::Lanczos3);
    scaled.save(dest)
}

pub async  fn scan_job(dirs: Vec<PathBuf>) -> i64 {
    let conn = match database::connection::open() {
        Ok(c) => c,
        Err(e) => {
            println!("打开数据库失败：{e}");
            return 0;
        }
    };
    let _ = database::init(&conn);
    let mut added = 0;

    for dir in dirs {
        for path in scan::scan_videos(&dir) {
            let file_path = path.to_string_lossy().to_string();
            if repository::exists_by_path(&conn, &file_path).unwrap_or(false) {
                continue;
            }
            let title = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .replace('_', " ");
            let duration = get_duration(&path);
            let media = Media {
                id: None,
                title,
                year: None,
                overview: None,
                media_type: MediaType::Personal,
                duration,
                rating: None,
                actors: vec![],
                poster_path: None,
                file_path,
            };
            if repository::insert_media(&conn, &media).is_ok() {
                // 封面
                if let Some(src) = scan::find_poster(&path) {
                    let id = conn.last_insert_rowid();
                    let _ = fs::create_dir_all(config::posters_dir());
                    let ext = src.extension().unwrap_or_default().to_string_lossy();
                    let dest = config::posters_dir().join(format!("{id}.jpg"));
                    if scan::make_thumbnail(&src, &dest).is_ok() {
                        repository::update_poster(&conn, id, &dest.to_string_lossy())
                            .unwrap_or_default()
                    }
                }
                added +=1;
            }
        }
    }
    added
}

fn get_duration(path: &Path) -> Option<String> {
    let output = std::process::Command::new("ffprobe")
        .args(["-v", "error", "-show_entries", "format=duration", "-of", "csv=p=0"])
        .arg(path)
        .output()
        .ok()?;
    let s = String::from_utf8_lossy(&output.stdout);
    let mut seconds = s.trim().parse::<f64>().ok().map(|m| m as i64)?;
    seconds = seconds.max(0);
    let h = seconds / 3600;
    let m = (seconds % 3600) / 60;
    let s = seconds % 60;
    Some(format!("{:02}:{:02}:{:02}", h, m, s))
}