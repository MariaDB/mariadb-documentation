use crate::url_to_path;

use crate::{
    req::{self, ScrapeClient},
    scrape, BASE_PATH,
};
use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub trait Crawler: Sized {
    fn ignore_existing_files(&self) -> bool;
    fn starting_urls(&mut self) -> VecDeque<String> {
        vec!["https://mariadb.com/kb/en/".into()].into()
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
            let html = match read_if_ignore(&self, &path) {
                Some(html) => html,
                None => match client.get(&next_url) {
                    Ok(result) => result.html,
                    Err(_err) => continue,
                },
            };
            fs::create_dir_all(path.parent().unwrap_or_else(|| panic!("{path:?}")))?;
            fs::write(&path, &html)?;
            let urls = scrape::scrape_urls(&html);
            queue.extend(urls);
        }

        Ok(())
    }
}

pub fn read_if_ignore<C: Crawler>(crawler: &C, path: &Path) -> Option<String> {
    if !crawler.ignore_existing_files() {
        return None;
    }
    fs::read_to_string(path).ok()
}
