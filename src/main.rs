mod app_args;
mod crawler;
mod logger;
mod method;
mod redirect;
mod req;
mod scrape;
mod standard_crawler;
mod starting_urls;

use app_args::AppArgs;
use crawler::Crawler;
use method::ScrapeMethod;
use scrape::trim_url;
use standard_crawler::StandardCrawler;
use std::{error::Error, path::PathBuf};

pub const BASE_PATH: &str = "../mariadb_archive";
pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

fn main() {
    logger::init();
    let args = AppArgs::read();
    if let Err(err) = run_crawler(args) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run_crawler(args: AppArgs) -> Result<(), Box<dyn Error>> {
    match args.scrape_method {
        ScrapeMethod::Standard => StandardCrawler::new(args.ignore_existing).crawl(),
        _ => todo!(),
    }
}

pub fn url_to_path(url: &str) -> PathBuf {
    let mut path = PathBuf::from(get_url_suffix(url));
    if path.extension().is_none() {
        path = path.join("index.html");
    }
    PathBuf::from(BASE_PATH).join(path)
}
pub fn get_url_suffix(mut url: &str) -> &str {
    if let Some(idx) = url.find('#').or_else(|| url.find('?')) {
        url = &url[..idx];
    }
    trim_url(url)
}
