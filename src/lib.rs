mod graceful_shutdown;
mod url_to_path;

use axum::Json;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
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
use url_to_path::{url_to_index_path, url_to_path};

const BASE_PATH: &str = "../mariadb_archive/";

pub async fn run() {
    let app = Router::new()
        .route("/", get(root))
        .route("/kb_urls.csv", get(get_kb_urls_csv))
        .route("/kb_urls.json", get(get_kb_urls_json))
        .route("/kb/*url", get(request_kb));
    let addr = "0.0.0.0:7032".parse().expect("Invalid Port");
    let server = Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(graceful_shutdown::shutdown_signal());
    println!("Listening on http://localhost:7032/");
    println!("Ctrl-C to exit.");
    server.await;
}

#[debug_handler]
async fn root(query: Query<ReqQuery>) -> Result<Response, StatusCode> {
    request_kb(extract::Path("/kb/en".to_owned()), query).await
}

async fn get_kb_urls_csv() -> Result<String, StatusCode> {
    fs::read_to_string("kb_urls.csv")
        .await
        .map_err(|_| StatusCode::NOT_FOUND)
}

async fn get_kb_urls_json() -> Result<Response, StatusCode> {
    let kb_urls = get_kb_urls_csv().await?;
    let mut reader = csv::Reader::from_reader(kb_urls.as_bytes());
    let rows: Result<Vec<HashMap<String, String>>, _> = reader.deserialize().collect();
    Ok(Json(rows.map_err(|_| StatusCode::NOT_FOUND)?).into_response())
}

#[derive(serde::Deserialize, Debug)]
pub struct ReqQuery {
    list: Option<String>,
}

#[debug_handler]
async fn request_kb(
    extract::Path(url): extract::Path<String>,
    Query(query): Query<ReqQuery>,
) -> Result<Response, StatusCode> {
    if query.list.is_some() {
        return request_kb_list(url).await;
    }
    let path = &PathBuf::from(BASE_PATH).join(url_to_index_path(&url));
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

async fn request_kb_list(url: String) -> Result<Response, StatusCode> {
    let path = PathBuf::from(BASE_PATH).join(url_to_path(&url));
    let Ok(mut dir) = fs::read_dir(&path).await else {
        eprintln!("path {path:?}");
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
