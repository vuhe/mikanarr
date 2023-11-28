use std::fs::File;
use std::path::Path;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

mod banner;

fn timer() -> fmt::time::ChronoLocal {
    fmt::time::ChronoLocal::new("%F %T%.3f".into())
}

pub(crate) fn load() {
    let log_path = match std::env::var("MK_APP_DATA") {
        Ok(it) => Path::new(&it).join("server.log"),
        Err(_) => Path::new("./data").join("server.log"),
    };
    let log_file = File::create(log_path).unwrap();
    let file_output = fmt::layer()
        .with_timer(timer())
        .with_ansi(false)
        .with_writer(log_file);

    tracing_subscriber::registry()
        .with(fmt::layer().with_timer(timer()))
        .with(file_output)
        .with(EnvFilter::from_default_env())
        .init();

    banner::print_banner();
}
