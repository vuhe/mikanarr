use anyhow::Result;
use sea_orm::entity::prelude::*;

use crate::res_data;

/// mikan 到 tmdb 的映射表
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "mikan_tmdb")]
pub struct Model {
    /// mikan bangumi id
    #[sea_orm(primary_key)]
    pub bangumi_id: String,
    /// tmdb id
    pub tmdb_id: i64,
    /// 是否为电影
    pub is_movie: bool,
    /// 影片季度（电影为空）
    pub season: String,
}

impl Model {
    pub async fn find_by_bangumi_id(bangumi_id: &str) -> Option<Self> {
        Entity::find_by_id(bangumi_id)
            .one(res_data().await)
            .await
            .ok()
            .and_then(|it| it)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
