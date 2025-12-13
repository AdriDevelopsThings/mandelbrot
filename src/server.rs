use axum::{extract, http::header, response::IntoResponse, routing::get, serve, Extension, Router};
use std::{env, net::SocketAddr, str::FromStr};
use tokio::net::TcpListener;

use crate::{
    data::Data,
    render::{get_key, render},
};

pub async fn get_tile(
    extract::Extension(data): extract::Extension<Data>,
    extract::Path((zoom, x, y)): extract::Path<(u32, u64, u64)>,
) -> impl IntoResponse {
    let key = get_key(zoom, x, y);
    let b = match data.get(&key).await {
        Some(b) => b,
        None => render(&data, x, y, zoom, Some(3600)).await,
    };

    ([(header::CONTENT_TYPE, "image/png")], b)
}

pub async fn index() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html")],
        include_str!("index.html"),
    )
}

pub async fn start_server(data: Data) {
    let app = Router::new()
        .route("/", get(index))
        .route("/{zoom}/{x}/{y}", get(get_tile))
        .layer(Extension(data));
    let listen_address =
        env::var("LISTEN_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1:8000"));
    let addr = SocketAddr::from_str(&listen_address).expect("Invalid listen address");
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Error while binding listener");
    println!("Server listening on {listen_address}");
    serve(listener, app)
        .await
        .expect("Error while serving http server");
}
