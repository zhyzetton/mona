use crate::database::model::library;
use crate::database::model::{MediaType, SourceType};
use crate::database::repository::Repository;
use crate::errors::AppError;
use crate::media::scanner::{utils, Scanner, VIDEO_EXTS};
use async_trait::async_trait;
use std::path::{Path, PathBuf};

pub struct LocalScanner {
    pub library_list: Vec<library::ActiveModel>,
}

impl LocalScanner {
    pub fn new(library_list: Vec<library::ActiveModel>) -> Self {
        Self { library_list }
    }

    /// 配置文件里的目录直接当本地库:名字取目录名,分类先用 Other
    /// (以后做了"添加媒体库"界面,就用 library::ActiveModel::new 自己传)
    pub fn from_dirs(dirs: Vec<PathBuf>) -> Self {
        Self {
            library_list: dirs
                .into_iter()
                .map(|dir| {
                    let name = dir
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_else(|| dir.to_string_lossy().into_owned());
                    library::ActiveModel::new(
                        SourceType::Local,
                        name,
                        MediaType::Other,
                        dir.to_string_lossy().into_owned(),
                    )
                })
                .collect(),
        }
    }
    /// 校验配置里的每个库:根目录必须存在;没入库的新库先写进数据库
    pub async fn check_library(&self, repo: &Repository) -> Result<(), AppError> {
        for library in &self.library_list {
            // ActiveModel 的字段是 ActiveValue 而不是裸 String,as_ref() 拿回 &String
            let root_path = library.root_path.as_ref();
            if !Path::new(root_path).exists() {
                return Err(AppError::Other(format!(
                    "Library root path not found: {root_path}"
                )));
            }
            // 已入库的不动,避免用配置默认值覆盖数据库里已有的设置
            if repo.get_library_by_root_path(root_path).await?.is_none() {
                repo.insert_library(library.clone()).await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Scanner for LocalScanner {
    async fn scan(&self, repo: &Repository) -> Result<(), AppError> {
        // 先校验根目录并把新库写进数据库,之后用 repo.get_enabled_libraries()
        // 拿带真实 id 的 library::Model 去挂 item / file
        self.check_library(repo).await?;

        // TODO: 逐库 walkdir,和 repo.get_relative_paths() 对比去重后 insert_item_with_files
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Local"
    }
}
