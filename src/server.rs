use axum::{
    body::Body, extract, http::header, response::IntoResponse, routing::get, serve, Router,
};
use std::{env, net::SocketAddr, path::PathBuf, str::FromStr};
use tokio::{fs::File, net::TcpListener};
use tokio_util::io::ReaderStream;

use crate::render::{get_tile_path, render};

pub async fn get_tile(
    extract::Path((zoom, x, y)): extract::Path<(u32, u64, u64)>,
) -> impl IntoResponse {
    let path = PathBuf::from(get_tile_path(zoom, x, y));
    if !path.exists() {
        render(x, y, zoom);
    }

    let file = File::open(path).await.unwrap();
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    ([(header::CONTENT_TYPE, "image/png")], body)
}

pub async fn index() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html")],
        include_str!("index.html"),
    )
}

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(index))
        .route("/{zoom}/{x}/{y}", get(get_tile));
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
