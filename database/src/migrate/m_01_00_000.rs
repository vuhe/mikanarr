use async_trait::async_trait;
use sea_orm_migration::prelude::*;

use super::*;

#[derive(DeriveMigrationName)]
pub(super) struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(config()).await?;
        manager.create_table(indexer()).await?;
        manager.create_table(downloader()).await?;
        manager.create_table(torrent()).await?;
        Ok(())
    }
}

fn config() -> TableCreateStatement {
    create_table("config")
        .if_not_exists()
        .col(column("key").string().not_null().primary_key())
        .col(column("value").string().not_null())
        .to_owned()
}

fn downloader() -> TableCreateStatement {
    create_table("downloader")
        .if_not_exists()
        .col(id().primary_key())
        .col(column("cat").integer().not_null())
        .col(column("name").string().not_null())
        .col(column("url").string().not_null())
        .col(column("username").string().null())
        .col(column("password").string().null())
        .col(column("download_dir").string().not_null().default(""))
        .to_owned()
}

fn indexer() -> TableCreateStatement {
    create_table("indexer")
        .if_not_exists()
        .col(id().primary_key())
        .col(column("name").string().not_null())
        .col(column("category").integer().not_null())
        .col(column("url").string().not_null())
        .col(column("enable").boolean().not_null().default(true))
        .to_owned()
}

fn torrent() -> TableCreateStatement {
    create_table("torrent")
        .if_not_exists()
        .col(column("id").string().not_null().primary_key())
        .col(column("name").string().not_null())
        .col(column("download_url").string().not_null())
        .col(column("pub_date").timestamp_with_time_zone().not_null())
        .col(column("title").string().not_null())
        .col(column("year").string().not_null())
        .col(column("is_movie").boolean().not_null())
        .col(column("season").string().not_null())
        .col(column("episode").string().not_null())
        .col(column("tmdb_id").big_integer().null())
        .col(column("tvdb_id").big_integer().null())
        .col(column("imdb_id").string().not_null())
        .to_owned()
}
