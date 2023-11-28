use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{Language, Tmdb};

#[derive(Serialize, Default, Debug)]
pub struct Param<'a> {
    pub api_key: &'a str,
    #[serde(skip)]
    pub series_id: i64,
    pub append_to_response: Option<&'a str>,
    pub language: Option<Language>,
}

#[derive(Deserialize, Debug)]
pub struct Resp {
    pub id: i64,
    #[serde(default)]
    pub backdrop_path: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub poster_path: String,
    #[serde(default)]
    pub external_ids: ExternalIds,
}

#[derive(Deserialize, Debug, Default)]
pub struct ExternalIds {
    pub tvdb_id: Option<i64>,
    pub imdb_id: Option<String>,
}

impl Tmdb {
    pub async fn tv_series_detail(&self, param: Param<'_>) -> Result<Resp> {
        let req = self.0.get(format!(
            "https://api.themoviedb.org/3/tv/{}",
            param.series_id
        ));
        let resp = req.query(&param).send().await?;
        Ok(resp.error_for_status()?.json().await?)
    }
}
