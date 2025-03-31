use axum::{routing::{get, post}, Router};
use routes::{health_check::health_check, insert_logs::insert_logs, query_logs::query_logs};
use tokio::net::TcpListener;

mod routes;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .route("/ingest", post(insert_logs))
        .route("/query", get(query_logs))
        .route("/health", get(health_check));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap()
}
