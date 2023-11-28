use anyhow::Result;
use sea_orm::entity::prelude::*;
use sea_orm::{IntoActiveModel, NotSet, QueryTrait};
use serde::{Deserialize, Serialize};

use crate::app_data;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Category {
    #[serde(rename = "rss")]
    #[sea_orm(num_value = 0)]
    Rss,
    #[serde(rename = "torznab")]
    #[sea_orm(num_value = 1)]
    Torznab,
}

#[derive(Deserialize)]
pub struct SearchParam {
    name: Option<String>,
    category: Option<Category>,
}

/// torrent 数据源（索引器）
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "indexer")]
pub struct Model {
    /// 索引器 id
    #[sea_orm(primary_key)]
    #[serde(default)]
    pub id: u32,
    /// 索引器名称
    pub name: String,
    /// 索引器类型
    pub category: Category,
    /// 索引器 url
    pub url: String,
    /// 是否启用
    pub enable: bool,
}

impl Model {
    pub async fn find_all_enable() -> Result<Vec<Self>> {
        Ok(Entity::find()
            .filter(Column::Enable.eq(true))
            .all(app_data().await)
            .await?)
    }

    pub async fn find_by_param(param: SearchParam) -> Result<Vec<Self>> {
        Ok(Entity::find()
            .apply_if(param.name, |it, v| {
                it.filter(Column::Name.like(format!("%{v}%")))
            })
            .apply_if(param.category, |it, v| it.filter(Column::Category.eq(v)))
            .all(app_data().await)
            .await?)
    }

    pub async fn add(self) -> Result<()> {
        let mut model = self.into_active_model().reset_all();
        model.id = NotSet;
        model.insert(app_data().await).await?;
        Ok(())
    }

    pub async fn modify(self) -> Result<()> {
        let model = self.into_active_model().reset_all();
        model.update(app_data().await).await?;
        Ok(())
    }

    pub async fn delete_by_id(id: u32) -> Result<()> {
        Entity::delete_by_id(id).exec(app_data().await).await?;
        Ok(())
    }

    pub async fn delete_all() -> Result<()> {
        Entity::delete_many().exec(app_data().await).await?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
