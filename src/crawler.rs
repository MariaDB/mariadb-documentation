use crate::redirect;
use crate::scrape::{format_url, scrape_urls};
use crate::url_to_path;

use crate::req::ScrapeClient;
use std::collections::HashSet;
use std::path::Path;
use std::{collections::VecDeque, error::Error, fs};

pub trait Crawler: Sized {
    fn ignore_existing_files(&self) -> bool;
    fn starting_urls(&mut self) -> VecDeque<String> {
        starting_urls()
    }
    fn crawl(mut self) -> Result<(), Box<dyn Error>> {
        let mut queue = self.starting_urls();
        let mut client = ScrapeClient::new();
        let mut completed_urls = HashSet::new();
        let mut count = 0;
        while let Some(next_url) = queue.pop_front() {
            if !completed_urls.insert(next_url.clone()) {
                continue;
            }
            count += 1;
            if count % 1000 == 0 {
                log::info!("Processed: {count}");
            }
            let Some(content) = read_and_write_content(&mut client, &self, &next_url) else {
                continue;
            };
            if let Ok(text) = String::from_utf8(content) {
                let urls = scrape_urls(&text);
                queue.extend(urls);
            }
        }

        Ok(())
    }
}

pub fn create_dir_and_write<C>(path: &Path, content: C) -> Result<(), std::io::Error>
where
    C: AsRef<[u8]>,
{
    fs::create_dir_all(path.parent().unwrap_or_else(|| panic!("{path:?}")))?;
    fs::write(path, content)
}

pub fn read_and_write_content<C>(
    client: &mut ScrapeClient,
    crawler: &C,
    url: &str,
) -> Option<Vec<u8>>
where
    C: Crawler,
{
    if crawler.ignore_existing_files() {
        if let Ok(content) = redirect::read(url) {
            return Some(content);
        }
    }
    let response = client.get(url).ok()?;
    let mut path = url_to_path(url);
    if response.directed_url != url {
        let redirect = redirect::create(&response.directed_url);
        let directed_path = url_to_path(&response.directed_url);
        create_dir_and_write(&path, redirect).unwrap_or_else(|_| panic!("{path:?}"));
        log::info!(
            "Redirected: '{}' -> '{}'",
            url.trim_start_matches("https://mariadb.com/"),
            response
                .directed_url
                .trim_start_matches("https://mariadb.com/")
        );
        path = directed_path;
    }
    create_dir_and_write(&path, &response.content).unwrap_or_else(|_| panic!("{path:?}"));
    Some(response.content)
}

pub fn starting_urls() -> VecDeque<String> {
    let mut kb_urls = crate::req::get_kb_urls().expect("msg");
    kb_urls.extend(
        ["https://mariadb.com/kb/en/"]
            .into_iter()
            .chain(include!("css_links.txt").into_iter())
            .map(format_url),
    );
    kb_urls.into()
}
