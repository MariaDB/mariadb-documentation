use std::path::{Path, PathBuf};

use tokio::{fs, io};

use crate::{url_to_path::url_to_index_path, BASE_PATH};

pub async fn read(path: &Path) -> Result<Vec<u8>, io::Error> {
    let content = fs::read(path).await?;
    let Ok(string) = std::str::from_utf8(&content) else {
        return Ok(content);
    };
    if string.starts_with("Redirect") {
        let redirected = string.trim_start_matches("Redirect ");
        let redirected_path = PathBuf::from(BASE_PATH).join(url_to_index_path(redirected));
        if let Ok(content) = fs::read(&redirected_path).await {
            return Ok(content);
        }
    }
    Ok(content)
}
