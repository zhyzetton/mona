use thiserror::Error;

#[derive(Error, Debug, Clone, serde::Serialize)]
pub enum AppError {
    #[error("配置文件读取失败: {0}")]
    FileOperation(String),
    
    #[error("数据库操作失败: {0}")]
    Database(String),

    #[error("扫描失败: {0}")]
    Scan(String),

    #[error("获取媒体失败: {0}")]
    GetMedia(String),

    #[error("Serde失败: {0}")]
    Serde(String),

    #[error("WebDav操作失败: {0}")]
    WebDav(String),

    #[error("未知错误: {0}")]
    Other(String)

}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Database(e.to_string())
    }
}