use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;

use time::{macros::format_description, UtcOffset};
use tracing_subscriber::fmt::time::OffsetTime;
// use tracing_subscriber::EnvFilter;

use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

//pub fn init_log(info_appender: NonBlocking, warn_appender: NonBlocking) {
pub struct Logger {
    info_appender: NonBlocking,
    _info_guard: WorkerGuard,
    warn_appender: NonBlocking,
    _warn_guard: WorkerGuard,
}

impl Logger {
    pub fn new() -> Logger {
        let info_appender = rolling::daily("/opt/logs", "info");
        let (info_appender, _info_guard) = tracing_appender::non_blocking(info_appender);
        let warn_appender = rolling::daily("/opt/logs", "warn");
        let (warn_appender, _warn_guard) = tracing_appender::non_blocking(warn_appender);

        Logger {
            info_appender,
            _info_guard,
            warn_appender,
            _warn_guard,
        }
    }
}

pub fn init_log(logger: Logger) {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "trace-example=debug,tower_http=debug")
    }

    // let info_appender = rolling::daily("/opt/logs", "info");
    // let (info_appender, _info_guard) = tracing_appender::non_blocking(info_appender);
    // let warn_appender = rolling::daily("/opt/logs", "warn");
    // let (warn_appender, _warn_guard) = tracing_appender::non_blocking(warn_appender);

    let mk_writer = logger
        .info_appender
        .with_min_level(Level::INFO)
        .and(logger.warn_appender.with_max_level(Level::WARN))
        .and(std::io::stdout.with_max_level(tracing::Level::TRACE));

    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );

    tracing_subscriber::fmt()
        // .with_env_filter(EnvFilter::from_default_env())
        .with_writer(mk_writer)
        .with_max_level(Level::TRACE)
        .with_timer(local_time)
        .init();
}

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    // build our application with a route

    // let info_appender = rolling::daily("/opt/logs", "info");
    // let (info_appender, _info_guard) = tracing_appender::non_blocking(info_appender);
    // let warn_appender = rolling::daily("/opt/logs", "warn");
    // let (warn_appender, _warn_guard) = tracing_appender::non_blocking(warn_appender);
    let logger = Logger::new();
    init_log(logger);

    let app = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    tracing::debug!("hello info");
    // log::info!("hello env log");
    Html("<h1>Hello, World!</h1>")
}
