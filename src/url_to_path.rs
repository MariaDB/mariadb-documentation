use std::path::PathBuf;
pub fn url_to_path(url: &str) -> PathBuf {
    let url_suffix = get_url_suffix(url);
    let mut path = PathBuf::from(url_suffix);
    if path.extension().is_none() {
        path = path.join("index");
    }
    path
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
