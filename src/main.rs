#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate futures;
extern crate regex;
extern crate reqwest;
extern crate toml;
extern crate url;
use axum::{extract::DefaultBodyLimit, routing::get, routing::post, Router};

mod api;
mod commands;
mod config;
mod errors;
mod github;
mod service;

#[cfg(test)]
mod testing;

use log::info;

const MAX_BODY_LENGTH: usize = 10 * 1024 * 1024;

#[tokio::main]
async fn main() {
    // initialize tracing
    //tracing_subscriber::fmt::init();

    info!("✨ May your hopes and dreams become reality ✨");
    config::load_config();

    // build our application with a single route
    let app = Router::new()
        .route("/check", get(service::check))
        .route("/github/events", post(service::github_event))
        .route("/gitlab/events", post(service::gitlab_event))
        .layer(DefaultBodyLimit::max(MAX_BODY_LENGTH));

    // run it with hyper on localhost:12345
    axum::Server::bind(&config::CONFIG.server.bindto.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
