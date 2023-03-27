mod redirect;
mod url_to_path;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use axum::{
    debug_handler,
    extract::{self, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router, Server,
};

use tokio::fs;
use url_to_path::{url_to_index_path, url_to_path};

const BASE_PATH: &str = "../mariadb_archive/";

pub async fn run(port: u32) {
    let app = Router::new()
        .route("/", get(root))
        .route("/kb_urls.csv", get(get_kb_urls_csv))
        .route("/kb_urls.csv/", get(get_kb_urls_csv))
        .route("/kb/*url", get(request_kb));
    let addr = format!("0.0.0.0:{port}").parse().expect("Invalid Port");
    println!("Listening on http://localhost:7032/");
    println!("Ctrl C to Exit.");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[debug_handler]
async fn root(query: Query<ReqQuery>) -> Result<Response, StatusCode> {
    request_kb(extract::Path("/kb/".to_owned()), query).await
}

async fn get_kb_urls_csv() -> Result<String, StatusCode> {
    fs::read_to_string("kb_urls.csv")
        .await
        .map_err(|_| StatusCode::NOT_FOUND)
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
    let extension = path
        .extension()
        .map_or(Some("html"), OsStr::to_str)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let content_type = content_type_from_extension(extension).ok_or(StatusCode::NOT_FOUND)?;
    content_builder(content_type, path).await
}

async fn request_kb_list(url: String) -> Result<Response, StatusCode> {
    let path = PathBuf::from(BASE_PATH).join(url_to_path(&url));
    let mut dir = fs::read_dir(&path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
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
    let bytes = redirect::read(path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let body = axum::body::Full::from(bytes);
    Response::builder()
        .header("content-type", content_type)
        .body(body)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(IntoResponse::into_response)
}
