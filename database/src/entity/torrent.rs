use anyhow::{Context, Result};
use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{ActiveValue, Condition, IntoActiveModel};

use crate::app_data;

#[derive(Default)]
pub struct SearchParam<'a> {
    pub imdb: Option<&'a str>,
    pub tvdb: Option<i64>,
    pub se: Option<&'a str>,
}

/// 监控的 torrent 信息
#[derive(Clone, Debug, PartialEq, Eq, Default, DeriveEntityModel)]
#[sea_orm(table_name = "torrent")]
pub struct Model {
    /// torrent hash
    #[sea_orm(primary_key)]
    pub id: String,
    /// torrent 名称
    pub name: String,
    /// torrent 下载地址
    pub download_url: String,
    /// torrent 发布时间
    pub pub_date: DateTimeWithTimeZone,
    /// 影片标题
    pub title: String,
    /// 影片年份
    pub year: String,
    /// 是否为电影
    pub is_movie: bool,
    /// 影片季度（电影为空）
    pub season: String,
    /// 影片集（电影为空）
    pub episode: String,
    /// tmdb id
    #[sea_orm(nullable)]
    pub tmdb_id: Option<i64>,
    /// tvdb id
    #[sea_orm(nullable)]
    pub tvdb_id: Option<i64>,
    /// imdb id
    pub imdb_id: String,
}

impl Model {
    /// 通过 id 查找
    pub async fn find_by_id(id: &str) -> Result<Self> {
        Entity::find_by_id(id.to_lowercase())
            .one(app_data().await)
            .await?
            .with_context(|| format!("torrent({id}) info is empty."))
    }

    /// 通过条件查找
    pub async fn filter(p: SearchParam<'_>) -> Result<Vec<Self>> {
        Ok(Entity::find()
            .filter(
                Condition::any()
                    .add_option(p.imdb.map(|it| Column::ImdbId.eq(it)))
                    .add_option(p.tvdb.map(|it| Column::TvdbId.eq(it)))
                    .add_option(p.se.map(|it| Column::Season.like(format!("%{it}%")))),
            )
            .all(app_data().await)
            .await?)
    }

    /// 此 torrent 是否存在
    pub async fn exist(&self) -> bool {
        Entity::find_by_id(self.id.to_lowercase())
            .one(app_data().await)
            .await
            .ok()
            .and_then(|it| it)
            .is_some()
    }

    pub async fn insert(self) -> Result<()> {
        let txn = app_data().await;
        let model = self.into_active_model();
        Entity::insert(model).exec(txn).await?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C: ConnectionTrait>(mut self, _: &C, _: bool) -> Result<Self, DbErr> {
        self.id = match self.id {
            ActiveValue::Set(it) => ActiveValue::Set(it.to_lowercase()),
            ActiveValue::Unchanged(it) => ActiveValue::Unchanged(it.to_lowercase()),
            ActiveValue::NotSet => ActiveValue::NotSet,
        };
        Ok(self)
    }
}
