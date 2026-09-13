use std::path::PathBuf;
use async_trait::async_trait;
use crate::database::repository::Repository;
use crate::errors::AppError;
use crate::media::model::Media;

pub mod local;
pub mod remote;
pub mod utils;

pub use local::LocalScanner;
pub use remote::RemoteScanner;


pub const VIDEO_EXTS: &[&str] = &["mp4", "mkv", "avi", "mov", "flv", "wmv", "ts"];
pub const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "webp"];

pub struct MetaInfo {
    pub duration: String,
    pub resolution: i32,
    pub file_size: String,
}

#[async_trait]
pub trait Scanner: Send + Sync {
    async fn scan(&self, repo: &Repository) -> Result<Vec<Media>, AppError>;
    fn name(&self) -> &'static str;
}

pub enum ScannerType {
    Local { local_dirs: Vec<PathBuf> },
    WebDav { url: String, username: String, password: String, root: String }
}

pub fn create_scanner(scanner_type: ScannerType) -> Result<Box<dyn Scanner>, AppError> {
    match scanner_type {
        ScannerType::Local { local_dirs } => Ok(Box::new(LocalScanner::new(local_dirs))),
        ScannerType::WebDav { url, username, password, root } => {
            let scanner = RemoteScanner::new(url, username, password, root)?;
            Ok(Box::new(scanner))
        }
    }
}