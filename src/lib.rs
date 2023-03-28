mod kb_urls;
mod redirect;
mod url_to_path;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use axum::extract::State;
use axum::{
    debug_handler,
    extract::{self, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router, Server,
};

use tokio::fs;

use kb_urls::{get_csv_diff, get_kb_urls_csv};
use url_to_path::{url_to_index_path, url_to_path};

const BASE_PATH: &str = "../mariadb_archive/";

#[derive(Clone)]
pub struct Config {
    pub port: u32,
    pub source: PathBuf,
}

#[tokio::main]
pub async fn run(args: &Config) {
    let app = Router::new()
        .route("/", get(root))
        .route("/kb_urls.csv", get(get_kb_urls_csv))
        .route("/kb_urls.csv/", get(get_kb_urls_csv))
        .route("/diff", get(get_csv_diff))
        .route("/diff/", get(get_csv_diff))
        .route("/kb/*url", get(request_kb))
        .with_state(args.clone());
    let addr = format!("0.0.0.0:{}", args.port)
        .parse()
        .expect("Invalid Port");
    println!("Listening on http://localhost:{}/", args.port);
    println!("Ctrl C to Exit.");
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[debug_handler]
async fn root(state: State<Config>, query: Query<ReqQuery>) -> Result<Response, StatusCode> {
    request_kb(state, extract::Path("/kb/".to_owned()), query).await
}

#[derive(serde::Deserialize, Debug)]
pub struct ReqQuery {
    list: Option<String>,
}

#[debug_handler]
async fn request_kb(
    State(state): State<Config>,
    extract::Path(url): extract::Path<String>,
    Query(query): Query<ReqQuery>,
) -> Result<Response, StatusCode> {
    if query.list.is_some() {
        return request_kb_list(&state.source, url)
            .await
            .map(IntoResponse::into_response);
    }
    let path = &state.source.join(url_to_index_path(&url));
    let extension = path
        .extension()
        .map_or(Some("html"), OsStr::to_str)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let content_type = content_type_from_extension(extension).ok_or(StatusCode::NOT_FOUND)?;
    content_builder(content_type, path).await
}

async fn request_kb_list(src: &Path, url: String) -> Result<String, StatusCode> {
    let path = src.join(url_to_path(&url));
    let mut dir = fs::read_dir(&path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let mut items = vec![];
    while let Ok(Some(item)) = dir.next_entry().await {
        if let Ok(false) = item.metadata().await.map(|metadata| metadata.is_file()) {
            items.push(item.file_name().to_string_lossy().to_string());
        }
    }
    Ok(items.join(","))
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
