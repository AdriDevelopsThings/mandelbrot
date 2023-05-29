use axum::{
    body::StreamBody, extract, http::header, response::IntoResponse, routing::get, Router, Server,
};
use std::{env, net::SocketAddr, path::PathBuf, str::FromStr};
use tokio::fs::File;
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
    let body = StreamBody::new(stream);

    ([(header::CONTENT_TYPE, "image/png")], body)
}

pub async fn index() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/html")], include_str!("index.html"))
}

pub async fn start_server() {

    let app = Router::new()
        .route("/", get(index))
        .route("/:zoom/:x/:y", get(get_tile));
    let listen_address =
        env::var("LISTEN_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1:8000"));
    let addr = SocketAddr::from_str(&listen_address).expect("Invalid listen address");
    println!("Server listening on {listen_address}");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Error while listening");
}
