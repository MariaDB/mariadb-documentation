use crate::ScrapeMethod;
use clap::{arg, Command};

#[derive(Debug)]
pub struct AppArgs {
    pub scrape_method: ScrapeMethod,
    pub clear: bool,
    pub ignore_existing: bool,
}

impl AppArgs {
    pub fn read() -> Self {
        let matches = Command::new("KbScraper")
            .arg(arg!([scrape_method] "[standard|recent|pdf|pdf_langs]"))
            .arg(arg!(-c --clear "Clears out the html directory"))
            .arg(arg!(-r --resume "Resumes the scrape ignoring already scraped directories"))
            .get_matches();

        let scrape_method_string = matches
            .get_one::<String>("scrape_method")
            .map_or_else(|| "standard", String::as_str);

        let scrape_method = match scrape_method_string.to_lowercase().as_str() {
            "" | "standard" => ScrapeMethod::Standard,
            "recent" => ScrapeMethod::RecentChanges,
            "pdf" => ScrapeMethod::Pdf,
            "pdf_langs" => ScrapeMethod::PdfLangs,
            other => panic!("Invalid Scrape Method: '{other}'"),
        };
        let clear = matches.get_one::<bool>("clear").copied().unwrap_or(false);
        let ignore_existing = matches.get_one::<bool>("resume").copied().unwrap_or(false);
        Self {
            scrape_method,
            clear,
            ignore_existing,
        }
    }
}
