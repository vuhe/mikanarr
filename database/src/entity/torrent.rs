use anyhow::{Context, Result};
use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::{ActiveValue, Condition, IntoActiveModel};

use crate::app_data;

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
    pub async fn find_by_query(
        imdb: Option<&str>,
        tvdb: Option<i64>,
        se: Option<&str>,
    ) -> Result<Vec<Self>> {
        let mut condition = Condition::any();
        if let Some(imdb) = imdb {
            condition = condition.add(Column::ImdbId.eq(imdb));
        }
        if let Some(tvdb) = tvdb {
            condition = condition.add(Column::TvdbId.eq(tvdb));
        }
        if let Some(se) = se {
            condition = condition.add(Column::Season.like(format!("%{se}%")));
        }

        Ok(Entity::find()
            .filter(condition)
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
        let exist = Entity::find_by_id(self.id.to_lowercase()).one(txn).await;
        let exist = exist.ok().and_then(|it| it);
        let model = self.into_active_model();
        match exist {
            None => drop(Entity::insert(model).exec(txn).await?),
            Some(_) => drop(Entity::update(model).exec(txn).await?),
        }
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
