use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{Language, Tmdb, TMDB_API};

#[derive(Serialize, Default, Debug)]
struct Param<'a> {
    api_key: &'a str,
    append_to_response: Option<&'a str>,
    language: Option<Language>,
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

pub struct TvSeriesDetail<'a> {
    client: Client,
    series_id: i64,
    param: Param<'a>,
}

impl<'a> TvSeriesDetail<'a> {
    pub fn append_to_response(mut self, ext: &'a str) -> Self {
        self.param.append_to_response = Some(ext);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.param.language = Some(language);
        self
    }

    pub async fn execute(self) -> Result<Resp> {
        let rul = format!("https://api.themoviedb.org/3/tv/{}", self.series_id);
        let req = self.client.get(rul);
        let resp = req.query(&self.param).send().await?;
        Ok(resp.error_for_status()?.json().await?)
    }
}

impl Tmdb {
    pub fn tv_series_detail(&self, series_id: i64) -> TvSeriesDetail {
        TvSeriesDetail {
            client: self.0.clone(),
            series_id,
            param: Param {
                api_key: TMDB_API,
                ..Default::default()
            },
        }
    }
}
