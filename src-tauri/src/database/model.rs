//! 表结构与实体定义(SeaORM)
//!
//! 表之间是逻辑外键(不建 FK 约束),引用完整性由 repository 维护:
//!   item.library_id      -> library.id  删 library 时连带删它的 item / file / 播放记录
//!   item.parent_id       -> item.id     树形:Series -> Season -> Episode
//!   file.item_id         -> item.id     Episode / Movie 直接挂 File
//!   play_history.file_id -> file.id     进度按文件记
//!   collection_item      关联 collection 与 item(多对多)
//!
//! 文件绝对路径 = library.root_path + file.relative_path

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum SourceType {
    #[sea_orm(string_value = "Local")]
    Local,
    #[sea_orm(string_value = "Webdav")]
    Webdav,
}

/// 库的内容分类
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum MediaType {
    #[sea_orm(string_value = "TVSeries")]
    TVSeries,
    #[sea_orm(string_value = "Movie")]
    Movie,
    #[sea_orm(string_value = "Anime")]
    Anime,
    #[sea_orm(string_value = "Variety")]
    Variety,
    #[sea_orm(string_value = "Documentary")]
    Documentary,
    #[sea_orm(string_value = "Other")]
    Other,
}

/// 条目在树里的角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum KindType {
    #[sea_orm(string_value = "Episode")]
    Episode,
    #[sea_orm(string_value = "Season")]
    Season,
    #[sea_orm(string_value = "Series")]
    Series,
    #[sea_orm(string_value = "Movie")]
    Movie,
    #[sea_orm(string_value = "Video")]
    Video,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum CollectionType {
    #[sea_orm(string_value = "MovieSeries")]
    MovieSeries,
    #[sea_orm(string_value = "Custom")]
    Custom,
}

pub mod library {
    use super::{MediaType, SourceType};
    use sea_orm::entity::prelude::*;
    use sea_orm::{NotSet, Set};
    use serde::Serialize;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "library")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub source: SourceType,
        pub name: String,
        pub media_type: MediaType,
        /// 本地目录或 webdav 路径,File.relative_path 相对它
        pub root_path: String,
        pub scrape: bool,
        pub private: bool,
        pub display_order: i64,
        pub enabled: bool,
        pub last_scan_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::item::Entity")]
        Item,
    }

    impl ActiveModel {
        /// 新建库的常见默认值:启用、要刮削、非私密、排序和扫描时间从 0 起
        pub fn new(
            source: SourceType,
            name: String,
            media_type: MediaType,
            root_path: String,
        ) -> Self {
            Self {
                id: NotSet,
                source: Set(source),
                name: Set(name),
                media_type: Set(media_type),
                root_path: Set(root_path),
                scrape: Set(true),
                private: Set(false),
                display_order: Set(0),
                enabled: Set(true),
                last_scan_at: Set(0),
            }
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod item {
    use super::KindType;
    use sea_orm::entity::prelude::*;
    use sea_orm::{NotSet, Set};
    use serde::Serialize;

    /// 逻辑媒体:Series -> Season -> Episode,或独立的 Movie / Video
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "item")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub library_id: i64,
        pub kind: KindType,
        pub parent_id: Option<i64>,
        pub title: String,
        pub overview: Option<String>,
        pub poster_path: Option<String>,
        pub backdrop_path: Option<String>,
        pub rating: Option<f32>,
        pub season: Option<i32>,
        pub episode: Option<i32>,
        pub external_id: Option<String>,
        pub scrape_status: bool,
        pub added_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::library::Entity",
            from = "Column::LibraryId",
            to = "super::library::Column::Id"
        )]
        Library,
        #[sea_orm(has_many = "super::file::Entity")]
        File,
        // 自引用:Season 指向 Series,Episode 指向 Season
        #[sea_orm(belongs_to = "Entity", from = "Column::ParentId", to = "Column::Id")]
        Parent,
    }

    // has_many 要求反向的 Related 实现,2.0 里 belongs_to 不再自动生成
    impl Related<super::library::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Library.def()
        }
    }

    impl ActiveModel {
        /// 扫描入库用的新条目:元数据字段留空,等刮削再填
        /// Episode/Movie/Video 直接挂文件;Series/Season 只是容器,parent_id 指向父节点
        pub fn new_scanned(
            library_id: i64,
            kind: KindType,
            title: String,
            parent_id: Option<i64>,
            season: Option<i32>,
            episode: Option<i32>,
            added_at: i64,
        ) -> Self {
            Self {
                id: NotSet,
                library_id: Set(library_id),
                kind: Set(kind),
                parent_id: Set(parent_id),
                title: Set(title),
                overview: Set(None),
                poster_path: Set(None),
                backdrop_path: Set(None),
                rating: Set(None),
                season: Set(season),
                episode: Set(episode),
                external_id: Set(None),
                scrape_status: Set(false),
                added_at: Set(added_at),
            }
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod file {
    use sea_orm::entity::prelude::*;
    use sea_orm::{NotSet, Set};
    use serde::Serialize;

    /// 物理文件,多个可以挂到同一个 Item 上(多版本/分片)
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "file")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: i64,
        pub item_id: i64,
        pub relative_path: String,
        pub size: i64,
        pub mtime: i64,
        pub duration: i64,
        pub width: i64,
        pub height: i64,
        /// 扫描时文件不在(外置盘未插、网盘掉线),记录保留
        pub is_missing: bool,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::item::Entity",
            from = "Column::ItemId",
            to = "super::item::Column::Id"
        )]
        Item,
    }

    impl Related<super::item::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Item.def()
        }
    }

    impl ActiveModel {
        /// 扫描入库用的新文件:item_id 留空,由 repository 插入时回填
        /// (单独调 insert_files 时才需要自己 set item_id)
        pub fn new_scanned(
            relative_path: String,
            size: i64,
            mtime: i64,
            duration: i64,
            width: i64,
            height: i64,
        ) -> Self {
            Self {
                id: NotSet,
                item_id: NotSet,
                relative_path: Set(relative_path),
                size: Set(size),
                mtime: Set(mtime),
                duration: Set(duration),
                width: Set(width),
                height: Set(height),
                is_missing: Set(false),
            }
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod play_history {
    use sea_orm::entity::prelude::*;
    use serde::Serialize;

    /// 每个文件一行:最后播放时间 + 进度
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "play_history")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub file_id: i64,
        pub played_at: i64,
        pub position: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::file::Entity",
            from = "Column::FileId",
            to = "super::file::Column::Id"
        )]
        File,
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod collection {
    use super::CollectionType;
    use sea_orm::entity::prelude::*;
    use serde::Serialize;

    /// 容器:系列电影(自动)或用户自建合集
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "collection")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub collection_type: CollectionType,
        pub title: String,
        pub poster_path: Option<String>,
        pub backdrop_path: Option<String>,
        pub external_id: Option<String>,
        pub created_at: i64,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::collection_item::Entity")]
        CollectionItem,
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod collection_item {
    use sea_orm::entity::prelude::*;
    use serde::Serialize;

    /// collection 与 item 的多对多关联,display_order 控制合集中的顺序
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
    #[sea_orm(table_name = "collection_item")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub collection_id: i64,
        #[sea_orm(primary_key, auto_increment = false)]
        pub item_id: i64,
        pub display_order: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::collection::Entity",
            from = "Column::CollectionId",
            to = "super::collection::Column::Id"
        )]
        Collection,
        #[sea_orm(
            belongs_to = "super::item::Entity",
            from = "Column::ItemId",
            to = "super::item::Column::Id"
        )]
        Item,
    }

    impl Related<super::collection::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Collection.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
