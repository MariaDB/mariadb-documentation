use crate::{request_kb_list, Config};
use axum::{extract::State, http::StatusCode};
use std::collections::HashSet;

pub async fn get_kb_urls_csv() -> Result<String, StatusCode> {
    tokio::fs::read_to_string("kb_urls.csv")
        .await
        .map_err(|_| StatusCode::NOT_FOUND)
}

#[derive(Debug, serde::Deserialize)]
struct KbItem {
    #[serde(rename = "URL")]
    url: String,
    #[serde(rename = "Duplicate slugs")]
    slugs: String,
}

async fn get_kb_url_items() -> Result<Vec<KbItem>, StatusCode> {
    let kb_urls = get_kb_urls_csv().await?;
    csv::Reader::from_reader(kb_urls.as_bytes())
        .deserialize::<KbItem>()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_kb_slug_urls() -> Result<HashSet<String>, StatusCode> {
    let mut urls = HashSet::new();
    for item in get_kb_url_items().await? {
        urls.insert(item.url);

        for slug in item.slugs.split(';') {
            urls.insert(format!("https://mariadb.com/kb/en/{slug}/"));
        }
    }
    Ok(urls)
}

pub async fn get_csv_diff(State(state): State<Config>) -> Result<String, StatusCode> {
    let kb_urls: HashSet<String> = get_kb_slug_urls().await?;
    let en_articles = request_kb_list(&state.source, "/kb/en/".to_owned()).await?;
    let en_articles_iter = en_articles.split(',').map(str::trim);

    let mut missing_urls = vec![];
    for path in en_articles_iter {
        let url = format!("https://mariadb.com/kb/en/{path}/");
        if !kb_urls.contains(&url) && !path.contains('+') {
            missing_urls.push(url);
        }
    }
    Ok(missing_urls.join("\n"))
}
