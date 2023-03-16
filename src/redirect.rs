use std::{fs, io};

use crate::url_to_path;

pub fn read(url: &str) -> Result<Vec<u8>, io::Error> {
    let content = fs::read(url_to_path(url))?;
    let Ok(string) = std::str::from_utf8(&content) else {
        return Ok(content);
    };
    if string.starts_with("Redirect") {
        let redirected = string.trim_start_matches("Redirect ");
        if let Ok(content) = fs::read(url_to_path(redirected)) {
            return Ok(content);
        }
    }
    Ok(content)
}
pub fn create(url: &str) -> String {
    format!("Redirect {url}")
}
