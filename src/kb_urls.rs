use crate::request_kb_list;
use axum::http::StatusCode;
use std::collections::HashSet;

pub async fn get_kb_urls_csv() -> Result<String, StatusCode> {
    tokio::fs::read_to_string("kb_urls.csv")
        .await
        .map_err(|_| StatusCode::NOT_FOUND)
}

#[derive(serde::Deserialize)]
struct KbItem {
    #[serde(rename = "URL")]
    url: String,
}

#[allow(clippy::unused_async)]
async fn get_kb_urls() -> Result<Vec<String>, StatusCode> {
    let kb_urls = get_kb_urls_csv().await?;
    csv::Reader::from_reader(kb_urls.as_bytes())
        .deserialize::<KbItem>()
        .map(|res| res.map(|kb_item| kb_item.url))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_csv_diff() -> Result<String, StatusCode> {
    let kb_urls: HashSet<String> = get_kb_urls().await?.into_iter().collect();
    let en_articles = request_kb_list("/kb/en/".to_owned()).await?;
    let en_articles_iter = en_articles.split(',').map(str::trim);

    let mut missing_urls = vec![];
    for path in en_articles_iter {
        let url = format!("https://mariadb.com/kb/en/{path}/");
        if !kb_urls.contains(&url) {
            missing_urls.push(url);
        }
    }
    Ok(missing_urls.join("\n"))
}
