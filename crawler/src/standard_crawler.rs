use crate::{
    app_args::AppArgs,
    crawler::{read_and_write_content, Crawler},
    req::ScrapeClient,
    scrape::scrape_urls,
    starting_urls::standard_starting_urls,
    Result,
};
use std::{collections::VecDeque, path::PathBuf};

pub struct StandardCrawler {
    queue: VecDeque<String>,
    resume: bool,
    root: PathBuf,
}
impl StandardCrawler {
    pub fn new(args: AppArgs) -> Self {
        let queue = standard_starting_urls(args.force, args.port);
        Self {
            queue,
            resume: args.ignore_existing,
            root: args.output,
        }
    }
}

impl Crawler for StandardCrawler {
    fn next_url(&mut self) -> Option<String> {
        self.queue.pop_front()
    }
    fn process_url(&mut self, url: &str, client: &mut ScrapeClient) -> Result<()> {
        let Some(content) = read_and_write_content(client, self.resume, &self.root, url) else {
            return Ok(());
        };
        if let Ok(text) = String::from_utf8(content) {
            let urls = scrape_urls(&text);
            self.queue.extend(urls);
        }
        Ok(())
    }
}
