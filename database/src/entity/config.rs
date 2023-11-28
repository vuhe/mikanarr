use anyhow::Result;
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use sha256::Sha256Digest;

use crate::app_data;

pub struct Config;

impl Config {
    pub async fn bangumi_default_status(&self) -> bool {
        get_by_key("bangumi_default_status")
            .await
            .and_then(|it| it.parse().ok())
            .unwrap_or(false)
    }

    pub async fn username(&self) -> String {
        get_by_key("username").await.unwrap_or("admin".into())
    }

    pub async fn password(&self) -> String {
        // 默认密码为 admin123
        let default_password = "240be518fabd2724ddb6f04eeb1da5967448d7e831c08c8fa822809f74c720a9";
        get_by_key("password")
            .await
            .unwrap_or(default_password.into())
    }

    pub async fn auth_intranet(&self) -> bool {
        get_by_key("auth_intranet")
            .await
            .and_then(|it| it.parse().ok())
            .unwrap_or(true)
    }
}

impl Config {
    pub async fn set_bangumi_default_status(&self, val: Option<bool>) -> Result<()> {
        save_config("bangumi_default_status", val).await
    }

    pub async fn set_username(&self, val: Option<String>) -> Result<()> {
        save_config("username", val).await
    }

    pub async fn set_password<D: Sha256Digest>(&self, val: Option<D>) -> Result<()> {
        let password = val.map(|it| sha256::digest(it));
        save_config("password", password).await
    }

    pub async fn set_auth_intranet(&self, val: Option<bool>) -> Result<()> {
        save_config("auth_intranet", val).await
    }
}

async fn get_by_key(key: &str) -> Option<String> {
    Entity::find_by_id(key)
        .one(app_data().await)
        .await
        .ok()?
        .map(|it| it.value)
}

async fn save_config<T: ToString>(key: &str, val: Option<T>) -> Result<()> {
    if let Some(val) = val {
        let model = Model {
            key: key.into(),
            value: val.to_string(),
        }
        .into_active_model()
        .reset_all();
        let txn = app_data().await;
        let exist = Entity::find_by_id(key).one(txn).await?;
        match exist {
            None => drop(Entity::insert(model).exec(txn).await?),
            Some(_) => drop(Entity::update(model).exec(txn).await?),
        }
    }
    Ok(())
}

/// 系统配置
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "config")]
pub struct Model {
    /// 键
    #[sea_orm(primary_key)]
    pub key: String,
    /// 值
    pub value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
