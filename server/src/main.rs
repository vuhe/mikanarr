use anyhow::{Context, Result};
use poem::listener::TcpListener;
use poem::Server;

mod logger;
mod router;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    logger::load();
    database::load().await;
    searcher::load();

    Server::new(TcpListener::bind("0.0.0.0:7810"))
        .run(router::route())
        .await
        .context("web service startup failed")
}
