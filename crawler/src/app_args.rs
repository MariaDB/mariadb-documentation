use std::path::PathBuf;

use crate::{ScrapeMethod, DEFAULT_ARCHIVE_PATH};
use clap::{arg, value_parser, Command};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone)]
pub struct AppArgs {
    pub scrape_method: ScrapeMethod,
    pub clear: bool,
    pub ignore_existing: bool,
    pub force: bool,
    pub port: u32,
    pub verbose: bool,
    pub output_log: Option<PathBuf>,
    pub output: PathBuf,
}

impl AppArgs {
    pub fn read() -> Self {
        let matches = Command::new("KbCrawler")
            .arg(arg!([scrape_method] "[standard|recent|pdf|pdf_langs]"))
            .arg(arg!(-c --clear "Clears the output directory"))
            .arg(arg!(-r --resume "Resumes the scrape ignoring already scraped directories"))
            .arg(arg!(-f --force "Forces the crawler to run even without the server"))
            .arg(arg!(-o --output <PATH>
                "Where to write the contents out to (default = '../archive'"))
            .arg(arg!(-v --verbose "Logs to stdout"))
            .arg(
                arg!(-l --outlog <PATH> "Output path for seperate logs")
                    .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(-p --port <PORT>
                    "which port to connect to the server with (default = 7032).")
                .value_parser(value_parser!(u32)),
            )
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
            .map_or(DEFAULT_ARCHIVE_PATH, String::as_str)
            .into();

        let clear = matches.get_one::<bool>("clear").copied().unwrap_or(false);
        let ignore_existing = matches.get_one::<bool>("resume").copied().unwrap_or(false);
        let force = matches.get_one::<bool>("force").copied().unwrap_or(false);
        let verbose = matches.get_one::<bool>("verbose").copied().unwrap_or(false);
        let output_log = matches.get_one::<PathBuf>("outlog").cloned();
        let port = matches.get_one::<u32>("port").copied().unwrap_or(7032);

        Self {
            scrape_method,
            clear,
            ignore_existing,
            force,
            port,
            verbose,
            output_log,
            output,
        }
    }
}
