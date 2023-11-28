use serde::Serialize;

use crate::DEFAULT_CATEGORY;

static TORRENT_FIELDS: [&str; 6] = [
    "id",
    "hashString",
    "percentDone",
    "isFinished",
    "status",
    "labels",
];

static DETAIL_FIELDS: [&str; 7] = [
    "id",
    "hashString",
    "percentDone",
    "isFinished",
    "status",
    "labels",
    "files",
];

#[derive(Debug, Serialize)]
#[serde(untagged, rename_all = "kebab-case")]
enum RequestArg<'a> {
    Empty,
    GetTorrent {
        ids: [&'a str; 1],
        fields: &'static [&'static str],
    },
    GetTorrentList {
        fields: &'static [&'static str],
    },
    AddTorrent {
        metainfo: String,
        download_dir: &'a str,
        labels: [&'a str; 1],
    },
}

/// 跳过 arg 序列化检查
fn skip_arguments(arg: &RequestArg) -> bool {
    matches!(arg, RequestArg::Empty)
}

/// 请求参数
#[derive(Debug, Serialize)]
pub(super) struct Request<'a> {
    method: &'static str,
    #[serde(skip_serializing_if = "skip_arguments")]
    arguments: RequestArg<'a>,
}

impl<'a> Request<'a> {
    pub(super) fn port_test() -> Self {
        Self {
            method: "port-test",
            arguments: RequestArg::Empty,
        }
    }

    pub(super) fn torrent_add(metainfo: String, download_dir: &'a str) -> Self {
        Self {
            method: "torrent-add",
            arguments: RequestArg::AddTorrent {
                metainfo,
                download_dir,
                labels: [DEFAULT_CATEGORY],
            },
        }
    }

    pub(super) fn torrent_get() -> Self {
        Self {
            method: "torrent-get",
            arguments: RequestArg::GetTorrentList {
                fields: &TORRENT_FIELDS,
            },
        }
    }

    pub(super) fn torrent_info(id: &'a str) -> Self {
        Self {
            method: "torrent-get",
            arguments: RequestArg::GetTorrent {
                ids: [id],
                fields: &DETAIL_FIELDS,
            },
        }
    }
}
