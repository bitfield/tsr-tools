use axum::{routing::get, Router};
use hello_3 as hello;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    println!("Listening on http://{addr}/");
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        Router::new().route("/", get(|| async { hello::world() })),
    )
    .await
    .unwrap();
}
