use std::sync::{Mutex, MutexGuard, OnceLock};

use cached::{Cached, TimedSizedCache};
use poem::http::header::AUTHORIZATION;
use poem::web::Json;
use poem::{handler, Endpoint, IntoResponse, Request, Response, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use database::entity::Config;

use super::ResultResp;

fn session() -> MutexGuard<'static, TimedSizedCache<String, ()>> {
    static SESSION: OnceLock<Mutex<TimedSizedCache<String, ()>>> = OnceLock::new();
    let mutex = SESSION.get_or_init(|| {
        let cache = TimedSizedCache::with_size_and_lifespan_and_refresh(5, 30 * 60, true);
        Mutex::new(cache)
    });
    mutex.lock().unwrap()
}

#[derive(Deserialize)]
struct LoginForm {
    account: String,
    password: String,
}

/// 登录接口，成功返回 token
#[handler]
pub(super) async fn login(Json(res): Json<LoginForm>) -> Json<ResultResp<String>> {
    // 421 账号错误
    if res.account != Config.username().await {
        return Json(ResultResp::from(421));
    }
    // 422 密码错误
    if sha256::digest(res.password) != Config.password().await {
        return Json(ResultResp::from(422));
    }

    tracing::info!("login user: {}", res.account);
    let token = Uuid::new_v4().to_string();
    session().cache_set(token.clone(), ());
    Json(ResultResp::from(Ok(token)))
}

#[derive(Serialize)]
struct UserInfo {
    username: String,
    version: String,
}

/// 用户信息接口，成功返回 username
#[handler]
pub(super) async fn username() -> Json<ResultResp<UserInfo>> {
    let info = UserInfo {
        username: Config.username().await,
        version: env!("CARGO_PKG_VERSION").into(),
    };
    Json(ResultResp::from(Ok(info)))
}

/// 鉴权白名单
static WHITE_LIST: [&str; 2] = ["/login", "/search/torznab"];

pub(super) async fn auth<E: Endpoint>(next: E, req: Request) -> Result<Response> {
    // 需要鉴权的接口
    if !WHITE_LIST.contains(&req.uri().path()) {
        let token = req.header(AUTHORIZATION);
        let token = token.and_then(|it| it.strip_prefix("Bearer "));
        let token_exist = token
            .map(|it| session().cache_get(it).is_some())
            .unwrap_or(false);

        if !token_exist {
            let invalid_token = Json(ResultResp::<()> {
                code: 401,
                message: None,
                data: None,
            });
            return Ok(invalid_token.into_response());
        }
    }

    next.call(req).await.map(|it| it.into_response())
}
