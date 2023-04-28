mod app_args;
mod crawler;
mod last_updated;
mod logger;
mod method;
mod recent_crawler;
mod redirect;
mod req;
mod scrape;
mod standard_crawler;
mod starting_urls;

use app_args::AppArgs;
use crawler::Crawler;
use last_updated::write_last_updated;
use method::ScrapeMethod;
use recent_crawler::RecentCrawler;
use scrape::trim_url;
use standard_crawler::StandardCrawler;
use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
};

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

pub const DEFAULT_ARCHIVE_PATH: &str = "../mariadb_kb_archive";

fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("{panic_info}");
    }));
    let args = AppArgs::read();
    logger::init(args.verbose, args.output_log.as_deref());
    if let Err(err) = run_crawler(&args) {
        eprintln!("{err}");
        std::process::exit(1);
    }
    if args.scrape_method.is_complete_scrape() {
        write_last_updated(&args.output).expect("Failed to update last updated");
    }
}

fn run_crawler(args: &AppArgs) -> Result<(), Box<dyn Error>> {
    log::info!("Selected: {:?}", &args.scrape_method);
    match args.scrape_method {
        ScrapeMethod::Standard => StandardCrawler::new(args.clone()).crawl(),
        ScrapeMethod::RecentChanges => RecentCrawler::new(args.output.clone()).crawl(),
        _ => unimplemented!(),
    }
}
pub fn url_to_path(root: &Path, url: &str) -> PathBuf {
    let mut path = PathBuf::from(get_url_suffix(url));
    if path.extension().is_none() {
        path = path.join("index.html");
    }
    PathBuf::from(root).join(path)
}
pub fn path_to_url(path: &Path, archive_path: &str) -> String {
    let path = path.to_string_lossy().to_string();
    let path = path
        .trim_start_matches(archive_path)
        .trim_end_matches("index.html")
        .replace('\\', "/");
    format!("https://mariadb.com/kb{path}")
}
pub fn get_url_suffix(mut url: &str) -> &str {
    if let Some(idx) = url.find('#').or_else(|| url.find('?')) {
        url = &url[..idx];
    }
    trim_url(url)
}

pub fn get_subpaths(mut path: &Path) -> Result<Vec<PathBuf>, io::Error> {
    if path
        .file_name()
        .map(|path| path.to_string_lossy() == "index.html")
        == Some(true)
    {
        if let Some(parent) = path.parent() {
            path = parent;
        };
    }
    get_subpaths_raw(path.to_owned())
}

fn get_subpaths_raw(path: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    if !path.is_dir() {
        return Ok(vec![path]);
    }
    let mut dir = std::fs::read_dir(path)?;
    let mut items = vec![];
    while let Some(Ok(item)) = dir.next() {
        let sub_items = get_subpaths_raw(item.path())?;
        items.extend_from_slice(&sub_items);
    }
    Ok(items)
}
