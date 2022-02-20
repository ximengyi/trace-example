use axum::{
    body::Bytes,
    http::{HeaderMap, Request},
    response::{Html, Response},
    routing::get,
    Router,
};
use std::{net::SocketAddr, time::Duration};
use tower_http::{ trace::TraceLayer};

use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
// use tracing::Span;
// use log::{debug, error, log_enabled, info, Level};
#[tokio::main]
async fn main() {

    // Set the RUST_LOG, if it hasn't been explicitly defined
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var(
    //         "RUST_LOG",
    //         "trace-example=debug,tower_http=debug",
    //     )
    // }


    let info_appender = rolling::daily("d:\\logs", "info");
    let (info_appender, _info_guard) = tracing_appender::non_blocking(info_appender);

    let warn_appender = rolling::daily("d:\\logs", "warn");
    let (warn_appender, _warn_guard) = tracing_appender::non_blocking(warn_appender);


    let mk_writer = info_appender
    .with_min_level(Level::INFO)
    .and(warn_appender.with_max_level(Level::WARN))
    .and(
        std::io::stdout
            .with_max_level(tracing::Level::TRACE)
        
    );

    // let subscriber = FmtSubscriber::;
    // tracing::subscriber::set_global_default(subscriber);
    tracing_subscriber::fmt()
    .with_writer(mk_writer)
    .with_max_level(Level::TRACE)
    //.with_timer(tracing_subscriber::fmt::time::time())
   // .with_timer(tracing_subscriber::fmt::time::Uptime::)
    .init();





    // env_logger::init();
    // build our application with a route
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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