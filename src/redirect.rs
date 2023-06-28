use crate::url_to_path;
use std::{fs, io, path::Path};

pub fn read(root: &Path, url: &str) -> Result<Vec<u8>, io::Error> {
    let content = fs::read(url_to_path(root, url))?;
    let Ok(string) = std::str::from_utf8(&content) else {
        return Ok(content);
    };
    if string.starts_with("Redirect") {
        let redirected = string.trim_start_matches("Redirect ");
        if let Ok(content) = fs::read(url_to_path(root, redirected)) {
            return Ok(content);
        }
    }
    Ok(content)
}
pub fn create(url: &str) -> String {
    format!("Redirect {url}")
}
