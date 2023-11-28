use std::sync::OnceLock;

use tokio_test::block_on;

use tmdb::search::tv::Param;
use tmdb::{Language, Tmdb};

const TMDB_API: &str = env!("MK_TMDB_API");

static CLIENT: OnceLock<Tmdb> = OnceLock::new();

#[test]
fn test_search_tv() {
    use tmdb::search::tv::Param;
    let tmdb = CLIENT.get_or_init(Tmdb::default);
    let req = tmdb.search_tv(Param {
        api_key: TMDB_API,
        query: "柯南",
        language: Some(Language::ZhCn),
        ..Default::default()
    });
    let resp = block_on(req).unwrap();
    println!("{resp:#?}");
}

#[test]
fn test_tv_series_details() {
    use tmdb::tv_series::details::Param;
    let tmdb = CLIENT.get_or_init(Tmdb::default);
    let req = tmdb.tv_series_detail(Param {
        api_key: TMDB_API,
        series_id: 30983,
        append_to_response: Some("external_ids"),
        ..Default::default()
    });
    let resp = block_on(req).unwrap();
    println!("{resp:#?}");
}
