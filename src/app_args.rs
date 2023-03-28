use std::path::PathBuf;

use crate::ScrapeMethod;
use clap::{arg, Command};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone)]
pub struct AppArgs {
    pub scrape_method: ScrapeMethod,
    pub clear: bool,
    pub ignore_existing: bool,
    pub force: bool,
    pub verbose: bool,
    pub output: PathBuf,
}

impl AppArgs {
    pub fn read() -> Self {
        let matches = Command::new("KbCrawler")
            .arg(arg!([scrape_method] "[standard|recent|pdf|pdf_langs]"))
            .arg(arg!(-c --clear "Clears the output directory"))
            .arg(arg!(-r --resume "Resumes the scrape ignoring already scraped directories"))
            .arg(arg!(-f --force "Forces the crawler to run even without the server"))
            .arg(arg!(-o --output <PATH> "Where to write the contents out to (default is '../mariadb_archive'"))
            .arg(arg!(-v --verbose "Logs to stdout"))
            .get_matches();

        let scrape_method_string = matches
            .get_one::<String>("scrape_method")
            .map_or_else(|| "standard", String::as_str);

        let scrape_method = match scrape_method_string.to_lowercase().as_str() {
            "" | "standard" => ScrapeMethod::Standard,
            "recent" => ScrapeMethod::RecentChanges,
            "pdf" => ScrapeMethod::Pdf,
            "pdf_langs" => ScrapeMethod::PdfLangs,
            other => {
                eprintln!("Invalid Scrape Method: '{other}'");
                std::process::exit(1);
            }
        };

        let output = matches
            .get_one::<String>("output")
            .map_or("../mariadb_archive", String::as_str)
            .into();

        let clear = matches.get_one::<bool>("clear").copied().unwrap_or(false);
        let ignore_existing = matches.get_one::<bool>("resume").copied().unwrap_or(false);
        let force = matches.get_one::<bool>("force").copied().unwrap_or(false);
        let verbose = matches.get_one::<bool>("verbose").copied().unwrap_or(false);

        Self {
            scrape_method,
            clear,
            ignore_existing,
            force,
            verbose,
            output,
        }
    }
}
