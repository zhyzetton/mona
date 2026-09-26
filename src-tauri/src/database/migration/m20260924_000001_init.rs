use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

/// 初始建表:library / item / file / play_history / collection / collection_item
/// 表之间是逻辑外键,不建 FK 约束,完整性由 repository 维护(见 database::model)
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Library::Table)
                    .if_not_exists()
                    .col(pk_auto(Library::Id))
                    .col(text(Library::Source))
                    .col(text(Library::Name))
                    .col(text(Library::MediaType))
                    .col(text(Library::RootPath))
                    .col(boolean(Library::Scrape).default(true))
                    .col(boolean(Library::Private).default(false))
                    .col(integer(Library::DisplayOrder).default(0))
                    .col(boolean(Library::Enabled).default(true))
                    .col(integer(Library::LastScanAt).default(0))
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_library_root_path")
                    .table(Library::Table)
                    .col(Library::RootPath)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(pk_auto(Item::Id))
                    .col(integer(Item::LibraryId))
                    .col(text(Item::Kind))
                    .col(integer_null(Item::ParentId))
                    .col(text(Item::Title))
                    .col(text_null(Item::Overview))
                    .col(text_null(Item::PosterPath))
                    .col(text_null(Item::BackdropPath))
                    .col(double_null(Item::Rating))
                    .col(integer_null(Item::Season))
                    .col(integer_null(Item::Episode))
                    .col(text_null(Item::ExternalId))
                    .col(boolean(Item::ScrapeStatus).default(false))
                    .col(integer(Item::AddedAt).default(0))
                    .take(),
            )
            .await?;
        for (name, column) in [
            ("idx_item_library", Item::LibraryId),
            ("idx_item_parent", Item::ParentId),
            ("idx_item_external_id", Item::ExternalId),
        ] {
            manager
                .create_index(
                    Index::create()
                        .name(name)
                        .table(Item::Table)
                        .col(column)
                        .to_owned(),
                )
                .await?;
        }

        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(pk_auto(File::Id))
                    .col(integer(File::ItemId))
                    .col(text(File::RelativePath))
                    .col(big_integer(File::Size).default(0))
                    .col(big_integer(File::Mtime).default(0))
                    .col(big_integer(File::Duration).default(0))
                    .col(integer(File::Width).default(0))
                    .col(integer(File::Height).default(0))
                    .col(boolean(File::IsMissing).default(false))
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_file_item_path")
                    .table(File::Table)
                    .col(File::ItemId)
                    .col(File::RelativePath)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PlayHistory::Table)
                    .if_not_exists()
                    .col(integer(PlayHistory::FileId).primary_key())
                    .col(big_integer(PlayHistory::PlayedAt))
                    .col(big_integer(PlayHistory::Position).default(0))
                    .take(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_play_history_played_at")
                    .table(PlayHistory::Table)
                    .col(PlayHistory::PlayedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Collection::Table)
                    .if_not_exists()
                    .col(pk_auto(Collection::Id))
                    .col(text(Collection::CollectionType))
                    .col(text(Collection::Title))
                    .col(text_null(Collection::PosterPath))
                    .col(text_null(Collection::BackdropPath))
                    .col(text_null(Collection::ExternalId))
                    .col(big_integer(Collection::CreatedAt).default(0))
                    .col(big_integer(Collection::UpdatedAt).default(0))
                    .take(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CollectionItem::Table)
                    .if_not_exists()
                    .col(integer(CollectionItem::CollectionId))
                    .col(integer(CollectionItem::ItemId))
                    .col(integer(CollectionItem::DisplayOrder).default(0))
                    // 复合主键必须写成表级约束,SQLite 不接受两个列级 PRIMARY KEY
                    .primary_key(
                        Index::create()
                            .col(CollectionItem::CollectionId)
                            .col(CollectionItem::ItemId),
                    )
                    .take(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for table in [
            CollectionItem::Table.into_iden(),
            Collection::Table.into_iden(),
            PlayHistory::Table.into_iden(),
            File::Table.into_iden(),
            Item::Table.into_iden(),
            Library::Table.into_iden(),
        ] {
            manager
                .drop_table(Table::drop().table(table).to_owned())
                .await?;
        }
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Library {
    Table,
    Id,
    Source,
    Name,
    MediaType,
    RootPath,
    Scrape,
    Private,
    DisplayOrder,
    Enabled,
    LastScanAt,
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
    LibraryId,
    Kind,
    ParentId,
    Title,
    Overview,
    PosterPath,
    BackdropPath,
    Rating,
    Season,
    Episode,
    ExternalId,
    ScrapeStatus,
    AddedAt,
}

#[derive(DeriveIden)]
enum File {
    Table,
    Id,
    ItemId,
    RelativePath,
    Size,
    Mtime,
    Duration,
    Width,
    Height,
    IsMissing,
}

#[derive(DeriveIden)]
enum PlayHistory {
    Table,
    FileId,
    PlayedAt,
    Position,
}

#[derive(DeriveIden)]
enum Collection {
    Table,
    Id,
    CollectionType,
    Title,
    PosterPath,
    BackdropPath,
    ExternalId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum CollectionItem {
    Table,
    CollectionId,
    ItemId,
    DisplayOrder,
}
