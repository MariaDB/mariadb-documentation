mod app_args;
mod crawler;
mod logger;
mod method;
mod redirect;
mod req;
mod scrape;
mod standard_crawler;
mod starting_urls;

use std::{error::Error, path::PathBuf};

use app_args::AppArgs;
use crawler::Crawler;
use method::ScrapeMethod;
use scrape::trim_url;
use standard_crawler::StandardCrawler;

pub const BASE_PATH: &str = "../mariadb_archive";
pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
    logger::init();
    let args = AppArgs::read();
    run_crawler(args).unwrap();
}

fn run_crawler(args: AppArgs) -> Result<(), Box<dyn Error>> {
    match args.scrape_method {
        ScrapeMethod::Standard => StandardCrawler::new(args.ignore_existing).crawl(),
        _ => todo!(),
    }
}

pub fn url_to_path(url: &str) -> PathBuf {
    let url_suffix = get_url_suffix(url);
    let mut path = PathBuf::from(url_suffix);
    if path.extension().is_none() {
        path = path.join("index.html");
    }
    PathBuf::from(BASE_PATH).join(path)
}
pub fn get_url_suffix(mut url: &str) -> &str {
    if let Some(idx) = url.find('?').or_else(|| url.find('#')) {
        url = &url[..idx];
    }
    trim_url(url)
}
