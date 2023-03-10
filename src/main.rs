mod url_to_path;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf}, collections::HashMap,
};
use tokio_util::io::ReaderStream;

use axum::{
    body::StreamBody,
    debug_handler, extract,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router, Server, Json,
};
use tokio::fs::{File, self};
use url_to_path::url_to_path;

const BASE_PATH: &str = "../mariadb_archive/";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/kb_urls.csv", get(get_kb_urls))
        .route("/*url", get(req));
    let addr = "0.0.0.0:7032".parse().unwrap();
    let server = Server::bind(&addr).serve(app.into_make_service());
    println!("Listening on http://localhost:7032/");
    server.await.unwrap();
}

#[debug_handler]
async fn root() -> Result<Response, StatusCode> {
    req(extract::Path("/kb/en".to_owned())).await
}

async fn get_kb_urls() -> Result<String, StatusCode> {
    fs::read_to_string("kb_urls.csv").await.map_err(|_| StatusCode::NOT_FOUND)
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
