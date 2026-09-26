use crate::database::repository::Repository;
use crate::errors::AppError;
use crate::media::model::{SourceType, Video};
use crate::media::scanner::{utils, Scanner, VIDEO_EXTS};
use async_trait::async_trait;
use rayon::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use walkdir::WalkDir;

pub struct LocalScanner {
    pub local_dirs: Vec<PathBuf>,
}

impl LocalScanner {
    pub fn new(local_dirs: Vec<PathBuf>) -> Self {
        Self { local_dirs }
    }

    // 获取视频文件夹中所有的视频路径
    fn scan_video_paths(dirs: &[PathBuf]) -> Vec<PathBuf> {
        dirs.iter()
            .flat_map(|dir| {
                WalkDir::new(dir)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.file_type().is_file())
                    .map(|e| e.path().to_path_buf())
                    .filter(|p| {
                        p.extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| VIDEO_EXTS.contains(&ext.to_lowercase().as_str()))
                            .unwrap_or(false)
                    })
            })
            .collect()
    }

    // 从路径解析视频元数据，返回 Video 模型;如果路径已存在则跳过
    fn process_video(path: &Path, existing_paths: &HashSet<String>) -> Option<Video> {
        let file_path = path.to_string_lossy().to_string();
        if existing_paths.contains(&file_path) {
            return None;
        }
        let metainfo = utils::get_metainfo(path).ok()?;
        let title = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .replace('_', " ");

        let (poster_path, detail_path) = if let Some(src) = utils::find_poster(path) {
            match utils::make_thumbnail(&src) {
                Ok((poster_dest, detail_dest)) => (
                    Some(poster_dest.to_string_lossy().to_string()),
                    Some(detail_dest.to_string_lossy().to_string()),
                ),
                Err(e) => {
                    eprintln!("生成缩略图失败： {}: {e}", src.display());
                    (None, None)
                }
            }
        } else {
            (None, None)
        };

        let tag = path
            .parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            .map(String::from)
            .unwrap_or_default();

        let added_at = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()?
            .as_secs() as i64;

        Some(Video {
            id: None,
            library_id: ,
            metadata_id: None,
            name: "",
            season: None,
            episode: None,
            path: "",
            duration: None,
            resolution: None,
            file_size: 0,
            added_at,
        })
    }
}

#[async_trait]
impl Scanner for LocalScanner {
    async fn scan(&self, repo: &Repository) -> Result<Vec<Video>, AppError> {
        // 1. 先在异步上下文里查询已存在的路径（repo 不进入闭包）
        let existing_paths: HashSet<String> = repo
            .get_all_paths()
            .unwrap_or_default()
            .into_iter()
            .collect();

        // 2. 克隆需要的字段，移入阻塞闭包
        let root_dirs = self.local_dirs.clone();

        // 3. spawn_blocking 闭包内不再触碰 self / repo
        let result = tokio::task::spawn_blocking(move || -> Result<Vec<Video>, AppError> {
            let all_videos = LocalScanner::scan_video_paths(&root_dirs);

            if all_videos.is_empty() {
                return Ok(vec![]);
            }

            let results: Vec<Video> = all_videos
                .par_iter()
                .filter_map(|path| LocalScanner::process_video(path, &existing_paths))
                .collect();

            Ok(results)
        })
        .await
        .map_err(|e| AppError::Scan(e.to_string()))??;

        // 4. 回到异步上下文，写入数据库
        if !result.is_empty() {
            repo.insert_media_batch(&result)?;
        }

        Ok(result)
    }

    fn name(&self) -> &'static str {
        "Local"
    }
}
