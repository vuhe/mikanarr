use anyhow::{bail, ensure, Context, Result};
use base64::Engine;
use reqwest::{RequestBuilder, Response as Resp, StatusCode};
use serde::de::DeserializeOwned;

use super::receiver::{AddTorrentResp, PortTestResp, Response, TorrentInfo, TorrentList};
use super::sender::Request;
use super::{client, Session, TR};

impl TR {
    pub(super) async fn port_test(&self) -> Result<()> {
        let req = self.build_req(Request::port_test());
        let resp: PortTestResp = self.rpc(req).await?;
        ensure!(resp.port_is_open, "Can't connect transmission port.");
        Ok(())
    }

    pub(super) async fn add_torrent(&self, torrent: &[u8], dir: &str) -> Result<String> {
        let metainfo = base64::engine::general_purpose::STANDARD.encode(torrent);
        let req = self.build_req(Request::torrent_add(metainfo, dir));
        let resp: AddTorrentResp = self.rpc(req).await?;
        Ok(resp.into_hash_string())
    }

    pub(super) async fn torrent_list(&self) -> Result<Vec<TorrentInfo>> {
        let req = self.build_req(Request::torrent_get());
        let resp: TorrentList = self.rpc(req).await?;

        // transmission rpc api 并未提供过滤参数，此处会获取所有 torrent 列表过滤
        let list = resp
            .torrents
            .into_iter()
            .filter(TorrentInfo::match_category);
        Ok(list.collect())
    }

    pub(super) async fn torrent_info(&self, id: &str) -> Result<TorrentInfo> {
        let req = self.build_req(Request::torrent_info(id));
        let resp: TorrentList = self.rpc(req).await?;
        let torrent = resp.torrents.into_iter().next();
        torrent.context("Can't find torrent info")
    }
}

impl TR {
    fn update_session(&self, resp: &Resp) -> Result<()> {
        let session_id = resp
            .headers()
            .get("X-Transmission-Session-Id")
            .context("Can't find Transmission-Session-Id")?;
        let session_id = session_id
            .to_str()
            .context("Can't parse Transmission-Session-Id")?;
        Session.set(self.id, session_id);
        Ok(())
    }

    fn build_req(&self, request: Request) -> RequestBuilder {
        let mut req = client().post(&self.url);
        let username = self.username.as_ref();
        if let Some(username) = username.filter(|it| !it.is_empty()) {
            let password = self.password.as_ref();
            req = req.basic_auth(username, password.filter(|it| !it.is_empty()));
        }
        if let Some(id) = Session.get(self.id).filter(|it| !it.is_empty()) {
            req = req.header("X-Transmission-Session-Id", id.as_ref());
        }
        req.json(&request)
    }

    async fn rpc<R: DeserializeOwned>(&self, req: RequestBuilder) -> Result<R> {
        // 未使用 stream body，clone 不会失败
        let first_req = req.try_clone().unwrap();
        let resp = first_req.send().await?;

        // 如果鉴权失败并返回 session 那么先设置 session 后再次尝试
        let resp = match resp.status() {
            StatusCode::CONFLICT => {
                self.update_session(&resp)?;
                let resp = req.send().await;
                resp.and_then(|it| it.error_for_status())?
            }
            _ => resp.error_for_status()?,
        };

        let resp: Response<R> = resp.json().await?;
        match resp.result.as_str() {
            "success" => Ok(resp.arguments),
            error => bail!("{error}"),
        }
    }
}
