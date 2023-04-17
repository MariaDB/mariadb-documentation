use crate::{redirect, req::ScrapeClient, url_to_path, Result};
use std::{collections::HashSet, fs, io, path::Path};

pub trait Crawler {
    fn next_url(&mut self) -> Option<String>;
    fn process_url(&mut self, url: &str, client: &mut ScrapeClient) -> Result<()>;
    fn crawl(&mut self) -> Result<()> {
        let mut client = ScrapeClient::default();
        let mut completed_urls = HashSet::new();
        while let Some(next_url) = self.next_url() {
            if completed_urls.contains(&next_url) {
                continue;
            }
            self.process_url(&next_url, &mut client)?;
            completed_urls.insert(next_url.clone());
        }
        Ok(())
    }
}

pub fn create_dir_and_write<C: AsRef<[u8]>>(path: &Path, content: C) -> Result<(), io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

pub fn read_and_write_content(
    client: &mut ScrapeClient,
    resume: bool,
    root: &Path,
    url: &str,
) -> Option<Vec<u8>> {
    if resume {
        if let Ok(content) = redirect::read(root, url) {
            return Some(content);
        }
    }
    let response = client.get(url).ok()?;
    let mut path = url_to_path(root, url);
    if response.directed_url != url {
        log::info!("Redirected: {url} -> {}", response.directed_url);
        let directed_path = url_to_path(root, &response.directed_url);
        let redirect = redirect::create(&response.directed_url);
        create_dir_and_write(&path, redirect).unwrap_or_else(|_| panic!("{path:?}"));
        path = directed_path;
    }
    create_dir_and_write(&path, &response.content).unwrap_or_else(|_| panic!("{path:?}"));
    Some(response.content)
}
