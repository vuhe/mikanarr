use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::OnceLock;

use once_cell::sync::Lazy as LazyLock;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use migrate::Migrator;

pub mod entity;
mod migrate;

/// 是否启用 sqlx 日志
static USE_SQLX_LOGGING: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("USE_SQLX_LOGGING")
        .map(|it| it.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
});

/// 资源数据库，预先嵌入的资源数据，只读不要更改
async fn res_data() -> &'static DatabaseConnection {
    static DATA: OnceLock<DatabaseConnection> = OnceLock::new();

    if let Some(data) = DATA.get() {
        return data;
    }

    let res_data_path = "./resources/res.sqlite";
    log::info!("res database path: `{}`", res_data_path);
    let res_data_url = format!("sqlite:file:{}?mode=ro", res_data_path);

    let mut opt = ConnectOptions::new(res_data_url);
    opt.sqlx_logging(*USE_SQLX_LOGGING);
    let database = match Database::connect(opt).await {
        Ok(it) => it,
        Err(e) => panic!("open res database error: {:#?}", e),
    };

    DATA.get_or_init(|| database)
}

/// 应用数据库，存放程序运行时数据
async fn app_data() -> &'static DatabaseConnection {
    static DATA: OnceLock<DatabaseConnection> = OnceLock::new();

    if let Some(data) = DATA.get() {
        return data;
    }

    let app_data_path = match std::env::var("MK_APP_DATA") {
        Ok(it) => Path::new(&it).join("data.sqlite"),
        Err(_) => Path::new("./data").join("data.sqlite"),
    };
    log::info!("app database path: `{}`", app_data_path.display());
    let app_data_url = format!("sqlite:file:{}?mode=rwc", app_data_path.display());

    let mut opt = ConnectOptions::new(app_data_url);
    opt.sqlx_logging(*USE_SQLX_LOGGING);
    let database = match Database::connect(opt).await {
        Ok(it) => it,
        Err(e) => panic!("open app database error: {:#?}", e),
    };
    if let Err(e) = Migrator::up(&database, None).await {
        panic!("app database migrate error: {:#?}", e)
    }

    DATA.get_or_init(|| database)
}

/// 尝试重设密码，遇到错误取消重设
async fn reset_password() {
    let app_data_path = match std::env::var("MK_APP_DATA") {
        Ok(it) => Path::new(&it).join("reset-password.txt"),
        Err(_) => Path::new("./data").join("reset-password.txt"),
    };

    // 路径不存在直接返回
    if !app_data_path.exists() {
        return;
    }

    let mut file = match File::open(app_data_path) {
        Ok(it) => BufReader::new(it),
        Err(e) => {
            log::warn!("open reset-password.txt fail: {}", e);
            return;
        }
    };

    let mut password = String::new();
    if let Err(e) = file.read_line(&mut password) {
        log::warn!("read reset-password.txt fail: {}", e);
        return;
    }

    let password = password.trim();
    if password.is_empty() {
        log::warn!("reset-password is empty");
        return;
    }

    match entity::Config.set_password(Some(password)).await {
        Ok(_) => log::info!("reset `{}` password: `{}`", "admin", password),
        Err(e) => log::warn!("save new password error: {}", e),
    }
}

/// 加载数据库连接
pub async fn load() {
    let _ = res_data().await;
    let _ = app_data().await;
    reset_password().await;
}
