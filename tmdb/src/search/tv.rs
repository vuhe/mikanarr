use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{Language, Tmdb, TMDB_API};

#[derive(Serialize, Default, Debug)]
struct Param<'a> {
    api_key: &'a str,
    query: &'a str,
    first_air_date_year: Option<&'a str>,
    include_adult: Option<bool>,
    language: Option<Language>,
    page: Option<u32>,
    year: Option<&'a str>,
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

pub struct SearchTv<'a> {
    client: Client,
    param: Param<'a>,
}

impl<'a> SearchTv<'a> {
    pub fn first_air_date_year(mut self, first_air_date_year: &'a str) -> Self {
        self.param.first_air_date_year = Some(first_air_date_year);
        self
    }

    pub fn include_adult(mut self) -> Self {
        self.param.include_adult = Some(true);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.param.language = Some(language);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.param.page = Some(page);
        self
    }

    pub fn year(mut self, year: &'a str) -> Self {
        self.param.year = Some(year);
        self
    }

    pub async fn execute(self) -> Result<Resp> {
        let req = self.client.get("https://api.themoviedb.org/3/search/tv");
        let resp = req.query(&self.param).send().await?;
        let mut resp: Resp = resp.error_for_status()?.json().await?;
        resp.results = resp
            .results
            .into_iter()
            .filter(|it| it.genre_ids.contains(&16))
            .collect();
        Ok(resp)
    }
}

impl Tmdb {
    pub fn search_tv<'a>(&self, query: &'a str) -> SearchTv<'a> {
        SearchTv {
            client: self.0.clone(),
            param: Param {
                api_key: TMDB_API,
                query,
                ..Default::default()
            },
        }
    }
}
