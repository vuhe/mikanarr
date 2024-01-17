use anyhow::{bail, Result};
use serde::de::{DeserializeOwned, IgnoredAny};
use serde::Serialize;
use serde_json::{json, Value};

use encode::base64_encode;

use super::receiver::{DownloadStatus, Response};
use super::{AR, CLIENT};

fn list_fields() -> Value {
    json!([
        "gid",
        "infoHash",
        "totalLength",
        "completedLength",
        "status",
    ])
}

fn detail_fields() -> Value {
    json!([
        "gid",
        "infoHash",
        "totalLength",
        "completedLength",
        "status",
        "files",
    ])
}

#[derive(Debug, Serialize)]
struct Request {
    jsonrpc: &'static str,
    id: &'static str,
    method: &'static str,
    params: Value,
}

impl AR {
    pub(super) async fn get_version(&self) -> Result<()> {
        let _: IgnoredAny = self.rpc("aria2.getVersion", |_| {}).await?;
        Ok(())
    }

    pub(super) async fn add_torrent(&self, torrent: &[u8], dir: &str) -> Result<String> {
        self.rpc("aria2.addTorrent", move |param| {
            param.push([base64_encode(torrent)].as_slice().into());
            param.push(json!({ "dir": dir }));
        })
        .await
    }

    pub(super) async fn tell_status(&self, id: &str) -> Result<DownloadStatus> {
        self.rpc("aria2.tellStatus", |param| {
            param.push(id.into());
            param.push(detail_fields());
        })
        .await
    }

    pub(super) async fn tell_active(&self) -> Result<Vec<DownloadStatus>> {
        self.rpc("aria2.tellActive", |param| {
            param.push(list_fields());
        })
        .await
    }

    pub(super) async fn tell_waiting(&self) -> Result<Vec<DownloadStatus>> {
        self.rpc("aria2.tellWaiting", |param| {
            param.push(0.into());
            param.push(1000.into());
            param.push(list_fields());
        })
        .await
    }

    pub(super) async fn tell_stopped(&self) -> Result<Vec<DownloadStatus>> {
        self.rpc("aria2.tellStopped", |param| {
            param.push(0.into());
            param.push(1000.into());
            param.push(list_fields());
        })
        .await
    }
}

impl AR {
    fn secret(&self) -> Option<&str> {
        self.secret.as_ref().map(String::as_str)
    }

    async fn rpc<F, T>(&self, method: &'static str, param_fn: F) -> Result<T>
    where
        F: FnOnce(&mut Vec<Value>) -> (),
        T: DeserializeOwned,
    {
        let mut param = Vec::with_capacity(3);
        if let Some(secret) = self.secret().filter(|it| !it.is_empty()) {
            param.push(format!("token:{secret}").into());
        }
        param_fn(&mut param);

        let req = CLIENT.post(&self.url).json(&Request {
            jsonrpc: "2.0",
            id: "mikanarr",
            method,
            params: param.into(),
        });
        // aira2 鉴权失败需要用户提供新的 secure
        // 不需要重复多次获取 session 等信息
        let resp = req.send().await?;

        let resp: Response<T> = resp.json().await?;
        match resp {
            Response::Ok { result } => Ok(result),
            Response::Error { error } => bail!("{}", error.message),
        }
    }
}
