use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn status() -> &'static str {
    "File Manager is alive"
}

#[tokio::main]
async fn main() {

    let app = Router::new()
    .route("/status", get(status));

    let addr = SocketAddr::from(([0, 0, 0, 0], 4000));
    println!("🚀 File Manager running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}