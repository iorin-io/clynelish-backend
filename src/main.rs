use axum;
use tokio::sync::Mutex;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

mod db;
mod handlers;
mod models;
mod routes;
mod serializers;

#[tokio::main]
async fn main() {
    println!("Starting server...");

    let env_filter = EnvFilter::from_default_env();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    let db_pool = db::establish_connection().await;

    let state = Arc::new(Mutex::new(db::AppState { db_pool }));

    let app = routes::create_routes(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8369));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
