use crate::scrape::{format_url, scrape_urls};
use crate::url_to_path;

use crate::{req::ScrapeClient, scrape, BASE_PATH};
use std::{
    collections::VecDeque,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub trait Crawler: Sized {
    fn ignore_existing_files(&self) -> bool;
    fn starting_urls(&mut self) -> VecDeque<String> {
        ["https://mariadb.com/kb/en/"]
            .into_iter()
            .chain(include!("css_links.txt").into_iter())
            .map(format_url)
            .collect()
    }
    fn is_valid_url(&mut self, url: &str) -> bool;
    fn crawl(mut self) -> Result<(), Box<dyn Error>> {
        let mut queue = self.starting_urls();
        let mut client = ScrapeClient::new();
        while let Some(next_url) = queue.pop_front() {
            if !self.is_valid_url(&next_url) {
                continue;
            }
            let path = PathBuf::from(BASE_PATH).join(url_to_path(&next_url));
            let content = match read_if_ignore(&self, &path) {
                Some(content) => content,
                None => match client.get(&next_url) {
                    Ok(result) => result.content,
                    Err(_err) => continue,
                },
            };
            fs::create_dir_all(path.parent().unwrap_or_else(|| panic!("{path:?}")))?;
            fs::write(&path, &content)?;
            if let Ok(text) = String::from_utf8(content) {
                let urls = scrape::scrape_urls(&text);
                queue.extend(urls);
            }
        }

        Ok(())
    }
}

pub fn read_if_ignore<C: Crawler>(crawler: &C, path: &Path) -> Option<Vec<u8>> {
    if !crawler.ignore_existing_files() {
        return None;
    }
    fs::read(path).ok()
}
