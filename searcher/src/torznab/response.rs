use std::collections::BTreeMap;

use anyhow::Result;
use bytes::{BufMut, BytesMut};
use poem::http::{header::CONTENT_TYPE, HeaderValue};
use poem::Response;
use rss::extension::{Extension, ExtensionMap};
use rss::{Channel, Guid, Item};

use database::entity::Torrent;

// language=XML
static CAPS_XML: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<caps>
  <server version="1.0" title="mikanarr" source="https://mikanani.me" />
  <limits max="50" default="20" />
  <searching>
    <search available="yes" supportedParams="q" />
    <tv-search available="yes" supportedParams="q,tvdbid,season" />
    <movie-search available="yes" supportedParams="q,imdbid" />
    <audio-search available="no" supportedParams="q" />
    <book-search available="no" supportedParams="q" />
  </searching>
  <categories>
    <category id="1000" name="TV" />
    <category id="2000" name="Movies" />
  </categories>
</caps>"#;

/// torznab caps 响应
pub(super) fn caps_resp() -> Response {
    let mut resp = Response::from(CAPS_XML);
    let xml_type = HeaderValue::from_static("application/xml; charset=utf-8");
    resp.headers_mut().insert(CONTENT_TYPE, xml_type);
    resp
}

/// 空 torznab rss
pub(super) fn empty_resp() -> Result<Response> {
    channel_resp(Vec::default())
}

/// 将 torrent 转换为 torznab rss
pub(super) fn channel_resp(torrents: Vec<Torrent>) -> Result<Response> {
    let channel = new_torznab(torrents);
    let writer = BytesMut::new().writer();
    let writer = channel.write_to(writer)?;
    let bytes = writer.into_inner().freeze();
    let mut response = Response::from(bytes);
    let rss_type = HeaderValue::from_static("application/rss+xml; charset=utf-8");
    response.headers_mut().insert(CONTENT_TYPE, rss_type);
    Ok(response)
}

// noinspection HttpUrlsUsage
/// 创建 torznab rss
fn new_torznab(torrents: Vec<Torrent>) -> Channel {
    let mut channel = Channel::default();

    // 设置 xml 的 torznab 命名空间
    channel.set_namespaces(BTreeMap::from([(
        "torznab".into(),
        "http://torznab.com/schemas/2015/feed".into(),
    )]));

    channel.title = "mikanarr search".into();
    channel.link = "https://github.com/vuhe/mikanarr".into();
    channel.description = "This search result is generated based on local database info.".into();
    channel.items = torrents.into_iter().map(torrent_into_item).collect();

    channel
}

fn torrent_into_item(torrent: Torrent) -> Item {
    let mut item = Item::default();

    // todo 补充 title 等字段

    // guid
    let mut guid = Guid::default();
    guid.value = torrent.id;
    item.guid = Some(guid);

    // todo size 值缺失
    // size 扩展
    let mut size_ext = Extension::default();
    size_ext.name = "torznab:attr".to_string();
    size_ext.attrs.insert("name".into(), "size".into());
    size_ext.attrs.insert("value".into(), "".into());

    // category 扩展
    let mut category_ext = Extension::default();
    category_ext.name = "torznab:attr".into();
    category_ext.attrs.insert("name".into(), "category".into());
    let category = if torrent.is_movie { "2000" } else { "1000" };
    category_ext.attrs.insert("value".into(), category.into());

    // 设置 torznab 的 item 扩展
    let mut item_map = BTreeMap::new();
    item_map.insert("torznab:attr".into(), vec![size_ext, category_ext]);
    let mut extension_map = ExtensionMap::default();
    extension_map.insert("torznab".into(), item_map);
    item.set_extensions(extension_map);

    item
}
