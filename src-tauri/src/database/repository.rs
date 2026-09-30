use crate::database::model::{collection, collection_item, file, item, library, play_history};
use crate::errors::AppError;
use sea_orm::sea_query::{Expr, OnConflict};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait,
};
use std::collections::{HashMap, HashSet};

pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    // ---------- library ----------
    //
    pub async fn insert_library(
        &self,
        model: library::ActiveModel,
    ) -> Result<library::Model, AppError> {
        Ok(library::Entity::insert(model)
            .on_conflict(
                OnConflict::column(library::Column::RootPath)
                    .update_columns([
                        library::Column::Source,
                        library::Column::Name,
                        library::Column::MediaType,
                        library::Column::Scrape,
                        library::Column::Private,
                        library::Column::DisplayOrder,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.db)
            .await?)
    }

    /// 按 root_path 去重:已存在则刷新配置,返回库记录
    pub async fn upsert_library(
        &self,
        model: library::ActiveModel,
    ) -> Result<library::Model, AppError> {
        Ok(library::Entity::insert(model)
            .on_conflict(
                OnConflict::column(library::Column::RootPath)
                    .update_columns([
                        library::Column::Source,
                        library::Column::Name,
                        library::Column::MediaType,
                        library::Column::Scrape,
                        library::Column::Private,
                        library::Column::DisplayOrder,
                        library::Column::Enabled,
                    ])
                    .to_owned(),
            )
            .exec_with_returning(&self.db)
            .await?)
    }

    pub async fn get_libraries(&self) -> Result<Vec<library::Model>, AppError> {
        Ok(library::Entity::find()
            .order_by_asc(library::Column::DisplayOrder)
            .all(&self.db)
            .await?)
    }

    /// 扫描用:启用中的库
    pub async fn get_enabled_libraries(&self) -> Result<Vec<library::Model>, AppError> {
        Ok(library::Entity::find()
            .filter(library::Column::Enabled.eq(true))
            .order_by_asc(library::Column::DisplayOrder)
            .all(&self.db)
            .await?)
    }

    pub async fn get_library_by_id(&self, id: i64) -> Result<library::Model, AppError> {
        library::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::Database(format!("library 不存在: id = {id}")))
    }

    pub async fn get_library_by_root_path(
        &self,
        root_path: &str,
    ) -> Result<Option<library::Model>, AppError> {
        Ok(library::Entity::find()
            .filter(library::Column::RootPath.eq(root_path))
            .one(&self.db)
            .await?)
    }

    pub async fn touch_library_scanned(&self, id: i64, at: i64) -> Result<(), AppError> {
        library::Entity::update_many()
            .col_expr(library::Column::LastScanAt, Expr::value(at))
            .filter(library::Column::Id.eq(id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    /// 删除媒体库:连带它的 item(含子节点)、file、播放记录、合集关联一起删
    /// 逻辑外键没有级联,顺序不能反
    pub async fn delete_library(&self, id: i64) -> Result<(), AppError> {
        let txn = self.db.begin().await?;

        let item_ids: Vec<i64> = item::Entity::find()
            .filter(item::Column::LibraryId.eq(id))
            .all(&txn)
            .await?
            .into_iter()
            .map(|row| row.id)
            .collect();

        if !item_ids.is_empty() {
            let file_ids: Vec<i64> = file::Entity::find()
                .filter(file::Column::ItemId.is_in(item_ids.clone()))
                .all(&txn)
                .await?
                .into_iter()
                .map(|row| row.id)
                .collect();

            if !file_ids.is_empty() {
                play_history::Entity::delete_many()
                    .filter(play_history::Column::FileId.is_in(file_ids))
                    .exec(&txn)
                    .await?;
            }
            collection_item::Entity::delete_many()
                .filter(collection_item::Column::ItemId.is_in(item_ids.clone()))
                .exec(&txn)
                .await?;
            file::Entity::delete_many()
                .filter(file::Column::ItemId.is_in(item_ids.clone()))
                .exec(&txn)
                .await?;
            item::Entity::delete_many()
                .filter(item::Column::Id.is_in(item_ids))
                .exec(&txn)
                .await?;
        }

        library::Entity::delete_by_id(id).exec(&txn).await?;
        txn.commit().await?;
        Ok(())
    }

    // ---------- item ----------

    pub async fn insert_item(&self, model: item::ActiveModel) -> Result<item::Model, AppError> {
        Ok(model.insert(&self.db).await?)
    }

    /// 一次写入"条目 + 它的文件",整体在一个事务里
    /// 文件上填的 item_id 会被新条目的 id 覆盖,调用方不用自己接
    pub async fn insert_item_with_files(
        &self,
        model: item::ActiveModel,
        files: Vec<file::ActiveModel>,
    ) -> Result<(item::Model, Vec<file::Model>), AppError> {
        let txn = self.db.begin().await?;

        let saved = model.insert(&txn).await?;
        let mut saved_files = Vec::with_capacity(files.len());
        for mut file in files {
            file.item_id = Set(saved.id);
            saved_files.push(file.insert(&txn).await?);
        }

        txn.commit().await?;
        Ok((saved, saved_files))
    }

    pub async fn get_item_by_id(&self, id: i64) -> Result<item::Model, AppError> {
        item::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::Database(format!("item 不存在: id = {id}")))
    }

    /// 库里所有条目
    pub async fn get_items_by_library(
        &self,
        library_id: i64,
    ) -> Result<Vec<item::Model>, AppError> {
        Ok(item::Entity::find()
            .filter(item::Column::LibraryId.eq(library_id))
            .order_by_asc(item::Column::Title)
            .all(&self.db)
            .await?)
    }

    /// 库里的顶层条目(剧集/电影本身,不含 Season/Episode),最近添加的在前
    pub async fn get_root_items(&self, library_id: i64) -> Result<Vec<item::Model>, AppError> {
        Ok(item::Entity::find()
            .filter(item::Column::LibraryId.eq(library_id))
            .filter(item::Column::ParentId.is_null())
            .order_by_desc(item::Column::AddedAt)
            .all(&self.db)
            .await?)
    }

    /// 树的一层:某个条目下面的季/集,按季集号排
    pub async fn get_children(&self, parent_id: i64) -> Result<Vec<item::Model>, AppError> {
        Ok(item::Entity::find()
            .filter(item::Column::ParentId.eq(parent_id))
            .order_by_asc(item::Column::Season)
            .order_by_asc(item::Column::Episode)
            .all(&self.db)
            .await?)
    }

    /// 条目 + 它挂的所有文件(一部电影有多个版本时都在这)
    pub async fn get_item_with_files(
        &self,
        id: i64,
    ) -> Result<(item::Model, Vec<file::Model>), AppError> {
        let model = self.get_item_by_id(id).await?;
        let files = self.get_files_by_item(id).await?;
        Ok((model, files))
    }

    pub async fn update_item(&self, model: item::Model) -> Result<item::Model, AppError> {
        Ok(model.into_active_model().update(&self.db).await?)
    }

    pub async fn set_scrape_status(&self, id: i64, done: bool) -> Result<(), AppError> {
        item::Entity::update_many()
            .col_expr(item::Column::ScrapeStatus, Expr::value(done))
            .filter(item::Column::Id.eq(id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    /// 刮削匹配查重
    pub async fn find_item_by_external_id(
        &self,
        external_id: &str,
    ) -> Result<Option<item::Model>, AppError> {
        Ok(item::Entity::find()
            .filter(item::Column::ExternalId.eq(external_id))
            .one(&self.db)
            .await?)
    }

    // ---------- file ----------

    pub async fn insert_file(&self, model: file::ActiveModel) -> Result<file::Model, AppError> {
        Ok(model.insert(&self.db).await?)
    }

    pub async fn insert_files(
        &self,
        files: Vec<file::ActiveModel>,
    ) -> Result<Vec<file::Model>, AppError> {
        let txn = self.db.begin().await?;
        let mut saved = Vec::with_capacity(files.len());
        for file in files {
            saved.push(file.insert(&txn).await?);
        }
        txn.commit().await?;
        Ok(saved)
    }

    pub async fn get_files_by_item(&self, item_id: i64) -> Result<Vec<file::Model>, AppError> {
        Ok(file::Entity::find()
            .filter(file::Column::ItemId.eq(item_id))
            .order_by_desc(file::Column::Height)
            .all(&self.db)
            .await?)
    }

    /// 扫描去重用:某个库里已入库的全部相对路径
    pub async fn get_relative_paths(&self, library_id: i64) -> Result<Vec<String>, AppError> {
        let item_ids: Vec<i64> = item::Entity::find()
            .filter(item::Column::LibraryId.eq(library_id))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| row.id)
            .collect();

        if item_ids.is_empty() {
            return Ok(vec![]);
        }

        Ok(file::Entity::find()
            .filter(file::Column::ItemId.is_in(item_ids))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| row.relative_path)
            .collect())
    }

    /// 这个库里有没有这个相对路径的文件(重扫时按路径认领)
    pub async fn find_file(
        &self,
        library_id: i64,
        relative_path: &str,
    ) -> Result<Option<file::Model>, AppError> {
        let candidates = file::Entity::find()
            .filter(file::Column::RelativePath.eq(relative_path))
            .all(&self.db)
            .await?;
        if candidates.is_empty() {
            return Ok(None);
        }

        let item_ids: Vec<i64> = candidates.iter().map(|row| row.item_id).collect();
        let ids_in_library: HashSet<i64> = item::Entity::find()
            .filter(item::Column::Id.is_in(item_ids))
            .filter(item::Column::LibraryId.eq(library_id))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| row.id)
            .collect();

        Ok(candidates
            .into_iter()
            .find(|row| ids_in_library.contains(&row.item_id)))
    }

    /// 播放时挑文件:没丢的里面清晰度最高的那个
    pub async fn resolve_playable_file(
        &self,
        item_id: i64,
    ) -> Result<Option<file::Model>, AppError> {
        Ok(file::Entity::find()
            .filter(file::Column::ItemId.eq(item_id))
            .filter(file::Column::IsMissing.eq(false))
            .order_by_desc(file::Column::Height)
            .all(&self.db)
            .await?
            .into_iter()
            .next())
    }

    pub async fn set_file_missing(&self, id: i64, missing: bool) -> Result<(), AppError> {
        file::Entity::update_many()
            .col_expr(file::Column::IsMissing, Expr::value(missing))
            .filter(file::Column::Id.eq(id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    /// 重扫时文件变了(mtime/size 不一致):刷新技术信息
    pub async fn refresh_file(
        &self,
        id: i64,
        size: i64,
        mtime: i64,
        duration: i64,
        width: i64,
        height: i64,
    ) -> Result<(), AppError> {
        file::Entity::update_many()
            .col_expr(file::Column::Size, Expr::value(size))
            .col_expr(file::Column::Mtime, Expr::value(mtime))
            .col_expr(file::Column::Duration, Expr::value(duration))
            .col_expr(file::Column::Width, Expr::value(width))
            .col_expr(file::Column::Height, Expr::value(height))
            .col_expr(file::Column::IsMissing, Expr::value(false))
            .filter(file::Column::Id.eq(id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    // ---------- play_history ----------

    /// 记录一次播放:按文件记,已有记录则刷新时间与进度
    /// 逻辑外键,先确认 file 存在
    pub async fn record_play(&self, file_id: i64, position: i64) -> Result<(), AppError> {
        let exists = file::Entity::find_by_id(file_id).one(&self.db).await?;
        if exists.is_none() {
            return Err(AppError::Database(format!("file 不存在: id = {file_id}")));
        }

        play_history::Entity::insert(play_history::ActiveModel {
            file_id: Set(file_id),
            played_at: Set(unix_now()),
            position: Set(position),
        })
        .on_conflict(
            OnConflict::column(play_history::Column::FileId)
                .update_columns([
                    play_history::Column::PlayedAt,
                    play_history::Column::Position,
                ])
                .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;
        Ok(())
    }

    /// 播放中更新进度(不动播放时间);还没记录过就什么也不做
    pub async fn update_position(&self, file_id: i64, position: i64) -> Result<(), AppError> {
        play_history::Entity::update_many()
            .col_expr(play_history::Column::Position, Expr::value(position))
            .filter(play_history::Column::FileId.eq(file_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn get_play_history(
        &self,
        file_id: i64,
    ) -> Result<Option<play_history::Model>, AppError> {
        Ok(play_history::Entity::find_by_id(file_id)
            .one(&self.db)
            .await?)
    }

    /// 最近观看:按最后播放时间倒序,同一个条目只出现一次
    pub async fn get_recent_items(&self, limit: u64) -> Result<Vec<item::Model>, AppError> {
        let histories = play_history::Entity::find()
            .order_by_desc(play_history::Column::PlayedAt)
            .limit(limit)
            .all(&self.db)
            .await?;
        if histories.is_empty() {
            return Ok(vec![]);
        }

        let file_ids: Vec<i64> = histories.iter().map(|row| row.file_id).collect();
        let file_to_item: HashMap<i64, i64> = file::Entity::find()
            .filter(file::Column::Id.is_in(file_ids))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| (row.id, row.item_id))
            .collect();

        let item_ids: Vec<i64> = file_to_item.values().copied().collect();
        let mut items: HashMap<i64, item::Model> = item::Entity::find()
            .filter(item::Column::Id.is_in(item_ids))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| (row.id, row))
            .collect();

        // 按播放顺序取,同一个条目只保留最近那次
        let mut seen = HashSet::new();
        let mut recent = Vec::new();
        for history in histories {
            if let Some(item_id) = file_to_item.get(&history.file_id) {
                if seen.insert(*item_id) {
                    if let Some(model) = items.remove(item_id) {
                        recent.push(model);
                    }
                }
            }
        }
        Ok(recent)
    }

    // ---------- collection ----------

    pub async fn insert_collection(
        &self,
        model: collection::ActiveModel,
    ) -> Result<collection::Model, AppError> {
        Ok(model.insert(&self.db).await?)
    }

    pub async fn get_collections(&self) -> Result<Vec<collection::Model>, AppError> {
        Ok(collection::Entity::find()
            .order_by_asc(collection::Column::Title)
            .all(&self.db)
            .await?)
    }

    pub async fn find_collection_by_external_id(
        &self,
        external_id: &str,
    ) -> Result<Option<collection::Model>, AppError> {
        Ok(collection::Entity::find()
            .filter(collection::Column::ExternalId.eq(external_id))
            .one(&self.db)
            .await?)
    }

    /// 删除合集,连带删掉成员关联(item 本身不动)
    pub async fn delete_collection(&self, id: i64) -> Result<(), AppError> {
        let txn = self.db.begin().await?;
        collection_item::Entity::delete_many()
            .filter(collection_item::Column::CollectionId.eq(id))
            .exec(&txn)
            .await?;
        collection::Entity::delete_by_id(id).exec(&txn).await?;
        txn.commit().await?;
        Ok(())
    }

    pub async fn add_item_to_collection(
        &self,
        collection_id: i64,
        item_id: i64,
        display_order: i64,
    ) -> Result<(), AppError> {
        collection_item::Entity::insert(collection_item::ActiveModel {
            collection_id: Set(collection_id),
            item_id: Set(item_id),
            display_order: Set(display_order),
        })
        .on_conflict(
            OnConflict::columns([
                collection_item::Column::CollectionId,
                collection_item::Column::ItemId,
            ])
            .update_columns([collection_item::Column::DisplayOrder])
            .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;
        Ok(())
    }

    pub async fn remove_item_from_collection(
        &self,
        collection_id: i64,
        item_id: i64,
    ) -> Result<(), AppError> {
        collection_item::Entity::delete_many()
            .filter(collection_item::Column::CollectionId.eq(collection_id))
            .filter(collection_item::Column::ItemId.eq(item_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    /// 合集成员,按 display_order 排
    pub async fn get_collection_items(
        &self,
        collection_id: i64,
    ) -> Result<Vec<item::Model>, AppError> {
        let item_ids: Vec<i64> = collection_item::Entity::find()
            .filter(collection_item::Column::CollectionId.eq(collection_id))
            .order_by_asc(collection_item::Column::DisplayOrder)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| row.item_id)
            .collect();

        if item_ids.is_empty() {
            return Ok(vec![]);
        }

        let mut items: HashMap<i64, item::Model> = item::Entity::find()
            .filter(item::Column::Id.is_in(item_ids.clone()))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|row| (row.id, row))
            .collect();

        Ok(item_ids
            .into_iter()
            .filter_map(|id| items.remove(&id))
            .collect())
    }
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}
