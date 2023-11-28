use reqwest::Client;
use serde::Serialize;

pub mod search;
pub mod tv_series;

/// tmdb 语言代码
#[derive(Serialize, Debug)]
pub enum Language {
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "ja-JP")]
    JaJp,
}

/// tmdb client
/// - 搜索仅返回动画类别
/// - [API doc](https://developer.themoviedb.org/docs)
#[derive(Default)]
pub struct Tmdb(Client);

impl From<Client> for Tmdb {
    fn from(value: Client) -> Self {
        Self(value)
    }
}
