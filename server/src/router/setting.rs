use anyhow::Result;
use poem::web::Json;
use poem::{get, handler, put, Route};
use serde::{Deserialize, Serialize};

use database::entity::Config;

use super::ResultResp;

pub(super) fn route() -> Route {
    Route::new()
        .nest("/info", get(info))
        .nest("/modify", put(modify))
}

#[derive(Serialize, Deserialize)]
struct Settings {
    bangumi_default_status: Option<bool>,
    username: Option<String>,
    password: Option<String>,
    auth_intranet: Option<bool>,
}

impl Settings {
    async fn new() -> Self {
        Self {
            bangumi_default_status: Some(Config.bangumi_default_status().await),
            username: Some(Config.username().await),
            password: None,
            auth_intranet: Some(Config.auth_intranet().await),
        }
    }

    async fn save(self) -> Result<()> {
        Config
            .set_bangumi_default_status(self.bangumi_default_status)
            .await?;
        Config.set_username(self.username).await?;
        Config.set_password(self.password).await?;
        Config.set_auth_intranet(self.auth_intranet).await?;
        Ok(())
    }
}

#[handler]
async fn info() -> Json<ResultResp<Settings>> {
    Json(ResultResp::from(Ok(Settings::new().await)))
}

#[handler]
async fn modify(Json(setting): Json<Settings>) -> Json<ResultResp<()>> {
    Json(ResultResp::from(setting.save().await))
}
