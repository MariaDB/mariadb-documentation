use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct KbItem {
    url: String,
}

pub fn csv_urls() -> Vec<String> {
    csv::Reader::from_path("../kb_urls.csv")
        .expect("Failed to Read: 'kb_urls.csv'")
        .deserialize::<KbItem>()
        .filter_map(Result::ok)
        .map(|item| item.url)
        .collect()
}
