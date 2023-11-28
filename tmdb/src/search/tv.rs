use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{Language, Tmdb};

#[derive(Serialize, Default, Debug)]
pub struct Param<'a> {
    pub api_key: &'a str,
    pub query: &'a str,
    pub first_air_date_year: Option<&'a str>,
    pub include_adult: Option<bool>,
    pub language: Option<Language>,
    pub page: Option<u32>,
    pub year: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct Resp {
    pub page: i64,
    pub results: Vec<TvInfo>,
}

#[derive(Deserialize, Debug)]
pub struct TvInfo {
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub original_name: String,
    #[serde(default)]
    pub poster_path: String,
    #[serde(default)]
    pub genre_ids: Vec<u32>,
}

impl Tmdb {
    pub async fn search_tv(&self, param: Param<'_>) -> Result<Resp> {
        let req = self.0.get("https://api.themoviedb.org/3/search/tv");
        let resp = req.query(&param).send().await?;
        let mut resp: Resp = resp.error_for_status()?.json().await?;
        resp.results = resp
            .results
            .into_iter()
            .filter(|it| it.genre_ids.contains(&16))
            .collect();
        Ok(resp)
    }
}
