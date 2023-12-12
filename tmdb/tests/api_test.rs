use std::sync::OnceLock;

use tokio_test::block_on;

use tmdb::{Language, Tmdb};

static CLIENT: OnceLock<Tmdb> = OnceLock::new();

#[test]
fn test_search_tv() {
    let tmdb = CLIENT.get_or_init(Tmdb::default);
    let req = tmdb.search_tv("柯南").language(Language::ZhCn).execute();
    let resp = block_on(req).unwrap();
    println!("{resp:#?}");
}

#[test]
fn test_tv_series_details() {
    let tmdb = CLIENT.get_or_init(Tmdb::default);
    let req = tmdb
        .tv_series_detail(30983)
        .append_to_response("external_ids")
        .execute();
    let resp = block_on(req).unwrap();
    println!("{resp:#?}");
}
