mod helper;
mod parse_keyword;
mod parse_number;
mod parse_text;
mod text;
mod tokenize;
mod tokens;

/// 解析的信息
#[derive(Debug, Default)]
pub struct Element {
    pub anime_season: Option<String>,
    pub anime_title: Option<String>,
    pub anime_type: Vec<String>,
    pub anime_year: Option<String>,
    pub audio_term: Vec<String>,
    pub episode_number: Option<String>,
    pub language: Vec<String>,
    pub other: Vec<String>,
    pub release_group: Option<String>,
    pub release_information: Vec<String>,
    pub release_version: Option<String>,
    pub source: Vec<String>,
    pub subtitles: Option<String>,
    pub streaming: Option<String>,
    pub video_resolution: Option<String>,
    pub video_term: Vec<String>,
    pub volume_number: Option<String>,
}

impl Element {
    pub fn parse(title: impl AsRef<str>) -> Self {
        let mut element = Element::default();
        let mut tokens = tokens::Tokens::new(title.into());

        tokenize::tokenizer(&mut tokens);
        parse_number::parse_year(&mut element, &tokens);
        parse_keyword::parse_keyword(&mut element, &tokens);
        parse_text::parse_release_group(&mut element, &tokens);
        parse_number::parse_episode(&mut element, &tokens);
        parse_number::parse_volume(&mut element, &tokens);
        parse_text::parse_title(&mut element, &tokens);

        element
    }
}
