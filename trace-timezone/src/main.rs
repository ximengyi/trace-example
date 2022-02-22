use axum::{
    body::Bytes,
    http::{HeaderMap, Request},
    response::{Html, Response},
    routing::get,
    Router,
};
use std::{hash::Hasher, io::Write, net::SocketAddr, time::Duration};
use tower_http::trace::TraceLayer;

use time::macros::format_description;
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::{time::FormatTime, writer::MakeWriterExt};

use tracing_subscriber::fmt::time::LocalTime;
// use time::format_description;
// use tracing_subscriber::fmt::time::Utc;
use time::OffsetDateTime;

use time::UtcOffset;

#[tokio::main]
async fn main() {
    let info_appender = rolling::daily("/opt/logs", "info");
    let (info_appender, _info_guard) = tracing_appender::non_blocking(info_appender);

    let warn_appender = rolling::daily("/opt/logs", "warn");
    let (warn_appender, _warn_guard) = tracing_appender::non_blocking(warn_appender);

    let mk_writer = info_appender
        .with_min_level(Level::INFO)
        .and(warn_appender.with_max_level(Level::WARN))
        .and(std::io::stdout.with_max_level(tracing::Level::TRACE));

    // let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));
    // dbg!(timer.clone());
    //let time = OffsetDateTime::now_local();
    // let timer = LocalTime::new(time::format_description::well_known::Rfc3339);
    let timer = LocalTime::new(format_description!("[hour]:[minute]:[second]"));
    //time::UtcOffset::from_hms(8, 0, 0)
    //let timer = LocalTime::new(UtcOffset::from_hms(8, 0, 0));
    tracing_subscriber::fmt()
        .with_writer(mk_writer)
        .with_max_level(Level::TRACE)
        .with_timer(timer)
        .init();

    let app = Router::new()
        .route("/", get(handler))
        // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
        // It provides good defaults but is also very customizable.
        //
        // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
        .layer(TraceLayer::new_for_http());

    // If you want to customize the behavior using closures here is how
    //
    // This is just for demonstration, you don't need to add this middleware twice

    // run it
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
