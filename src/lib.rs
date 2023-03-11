mod graceful_shutdown;
mod url_to_path;

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio_util::io::ReaderStream;

use axum::{
    body::StreamBody,
    debug_handler,
    extract::{self, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router, Server,
};
use tokio::fs::{self, File};
use url_to_path::url_to_path;

const BASE_PATH: &str = "../mariadb_archive/";

pub async fn run() {
    let app = Router::new()
        .route("/", get(root))
        .route("/kb_urls.csv", get(get_kb_urls))
        .route("/kb/*url", get(req));
    let addr = "0.0.0.0:7032".parse().expect("Invalid Port");
    let server = Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(graceful_shutdown::shutdown_signal());
    println!("Listening on http://localhost:7032/");
    println!("Ctrl-C to exit.");
    server.await;
}

#[debug_handler]
async fn root() -> Result<Response, StatusCode> {
    req(
        extract::Path("/kb/en".to_owned()),
        Query(ReqQuery {
            list: Some(String::new()),
        }),
    )
    .await
}

async fn get_kb_urls() -> Result<String, StatusCode> {
    fs::read_to_string("kb_urls.csv")
        .await
        .map_err(|_| StatusCode::NOT_FOUND)
}

#[derive(serde::Deserialize)]
pub struct ReqQuery {
    list: Option<String>,
}

#[debug_handler]
async fn req(
    extract::Path(url): extract::Path<String>,
    Query(query): Query<ReqQuery>,
) -> Result<Response, StatusCode> {
    if query.list.is_some() {
        return req_list(url).await;
    }
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

async fn req_list(url: String) -> Result<Response, StatusCode> {
    let path = url_to_path(&url);
    let Some(parent) = path.parent() else {
        eprintln!("path {path:?}!");
        return Err(StatusCode::NOT_FOUND);
    };
    let cwd = PathBuf::from(BASE_PATH);
    let Ok(mut dir) = fs::read_dir(cwd.join(parent)).await else {
        eprintln!("parent {parent:?}");
        return Err(StatusCode::NOT_FOUND);
    };
    let mut items = vec![];
    while let Ok(Some(item)) = dir.next_entry().await {
        if let Ok(false) = item.metadata().await.map(|metadata| metadata.is_file()) {
            items.push(item.file_name().to_string_lossy().to_string());
        }
    }
    Ok(items.join(",").into_response())
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
