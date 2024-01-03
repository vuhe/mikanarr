use once_cell::sync::Lazy as LazyLock;
use tokio_test::block_on;

use tmdb::{Language, Tmdb};

static TMDB: LazyLock<Tmdb> = LazyLock::new(Tmdb::default);

#[test]
fn test_search_tv() {
    let req = TMDB.search_tv("柯南").language(Language::ZhCn).execute();
    let resp = block_on(req).unwrap();
    println!("{resp:#?}");
}

#[test]
fn test_tv_series_details() {
    let req = TMDB
        .tv_series_detail(30983)
        .append_to_response("external_ids")
        .execute();
    let resp = block_on(req).unwrap();
    println!("{resp:#?}");
}
