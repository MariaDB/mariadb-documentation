use std::path::PathBuf;

pub fn url_to_index_path(url: &str) -> PathBuf {
    let path = url_to_path(url);
    if path.extension().is_some() {
        return path;
    }
    path.join("index.html")
}

pub fn url_to_path(url: &str) -> PathBuf {
    PathBuf::from(get_url_suffix(url))
}
pub fn get_url_suffix(mut url: &str) -> &str {
    if let Some(idx) = url.find('?').or_else(|| url.find('#')) {
        url = &url[..idx];
    }
    url.trim_start_matches("https://")
        .trim_start_matches('/')
        .trim_start_matches("mariadb.com/")
        .trim_start_matches("kb/")
}
