use thiserror::Error;

#[derive(Error, Debug, Clone, serde::Serialize)]
pub enum AppError {

    #[error("创建文件失败: {0}")]
    CreateFile(String),

    #[error("配置文件读取失败: {0}")]
    FileOperation(String),
    
    #[error("数据库操作失败: {0}")]
    Database(String),

    #[error("扫描失败: {0}")]
    Scan(String),

    #[error("未知错误: {0}")]
    Other(String)

}