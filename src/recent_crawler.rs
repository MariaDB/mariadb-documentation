use std::path::{Path, PathBuf};

use crate::{
    crawler::{read_and_write_content, Crawler},
    get_subpaths,
    last_updated::read_last_updated,
    path_to_url,
    req::ScrapeClient,
    scrape::{format_url, valid_url},
    url_to_path, Result,
};
use chrono::NaiveDate;

pub struct RecentCrawler {
    queue: Vec<String>,
    root: PathBuf,
}
impl RecentCrawler {
    pub fn new(root: PathBuf) -> Self {
        let last_updated = read_last_updated();
        let queue = get_recent_urls_recursive(&root, last_updated);
        Self { queue, root }
    }
}

impl Crawler for RecentCrawler {
    fn next_url(&mut self) -> Option<String> {
        self.queue.pop()
    }
    fn process_url(&mut self, url: &str, client: &mut crate::req::ScrapeClient) -> Result<()> {
        read_and_write_content(client, false, &self.root, url);
        Ok(())
    }
}

fn get_recent_urls_recursive(root: &Path, last_updated: NaiveDate) -> Vec<String> {
    let mut urls = vec![];
    for url in get_recent_urls(root, last_updated) {
        let path = url_to_path(root, &url);
        let paths = get_subpaths(&path).unwrap_or_else(|_| panic!("Failed to read: {path:?}"));
        urls.extend(
            paths
                .into_iter()
                .map(|path| path_to_url(&path, root.to_string_lossy().to_string().as_str())),
        );
        urls.push(url);
    }
    urls
}

fn get_recent_urls(root: &Path, last_updated: NaiveDate) -> Vec<String> {
    let client = &mut ScrapeClient::new();
    let mut recent_articles = vec![];
    for page_num in 1.. {
        let url = format!("https://mariadb.com/kb/+changes/?p={page_num}");
        let content =
            read_and_write_content(client, false, root, &url).expect("Failed to read changes");
        let html = String::from_utf8(content).expect("Changes page was not valid utf-8");
        let all_articles: Vec<_> = scrape_recent_article_urls(&html).collect();
        let num_all_articles = all_articles.len();
        let new_articles: Vec<_> = all_articles
            .into_iter()
            .filter(|(_url, date)| *date >= last_updated)
            .map(|(url, _date)| url)
            .collect();
        let new_articles_len = new_articles.len();
        recent_articles.extend(new_articles);
        if num_all_articles > new_articles_len {
            break;
        }
    }
    recent_articles
}

fn scrape_recent_article_urls(html: &str) -> impl Iterator<Item = (String, NaiveDate)> + '_ {
    let scraped_urls = scrape_recent_article_urls_raw(html);
    scraped_urls
        .map(|(url, date)| (format_url(url), date))
        .filter(|(url, _date)| valid_url(url))
}

fn scrape_recent_article_urls_raw(html: &str) -> impl Iterator<Item = (String, NaiveDate)> + '_ {
    let re_urls = lazy_regex::regex!(
        r#"<li class="history_item" value="\d+">Article <a href="(?:(?:https://)?mariadb.com)?/?kb/([^"]*)">"#
    );
    let re_dates = lazy_regex::regex!(r#"<span class="datetime" title="([^"]+)">"#);
    let urls = re_urls.captures_iter(html).map(|cap| cap[1].to_owned());
    let dates = re_dates
        .captures_iter(html)
        .map(|cap| cap[1].to_owned())
        .map(|s| {
            let (first, _second) = s.split_once(' ').unwrap();
            first.parse::<NaiveDate>().unwrap()
        });
    urls.zip(dates)
}
