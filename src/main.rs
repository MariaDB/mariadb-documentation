mod url_to_path;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio_util::io::ReaderStream;

use axum::{
    body::StreamBody,
    debug_handler, extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router, Server,
};
use url_to_path::url_to_path;
use tokio::fs::File;

const BASE_PATH: &str = "../html/";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/*url", get(req));
    let addr = "0.0.0.0:7032".parse().unwrap();
    let server = Server::bind(&addr).serve(app.into_make_service());
    println!("Listening on {addr}");
    server.await.unwrap();
}

#[debug_handler]
async fn req(extract::Path(url): extract::Path<String>) -> Result<Response, StatusCode> {
    let path = &PathBuf::from(BASE_PATH).join(url_to_path(&url));
    let extension = match path.extension().map_or(Some("html"), OsStr::to_str) {
        Some(extension) => extension,
        None => return Err(StatusCode::BAD_REQUEST),
    };
    let content_type = match content_type_from_extension(extension) {
        Some(extension) => extension,
        None => return Err(StatusCode::NOT_FOUND), // TODO - Status Code
    };
    content_builder(content_type, path).await
}

fn content_type_from_extension(extension: &str) -> Option<&'static str> {
    let content_type = match extension {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "png" => "image/png; charset=utf-8",
        _ => return None,
    };
    Some(content_type)
}

async fn content_builder(content_type: &str, path: &Path) -> Result<Response, StatusCode> {
    debug_assert!(content_type.contains("charset=utf-8"));
    let file = File::open(path).await.map_err(|_| StatusCode::NOT_FOUND)?;
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);
    Ok(Response::builder()
        .header("content-type", content_type)
        .body(body)
        .unwrap()
        .into_response())
}
