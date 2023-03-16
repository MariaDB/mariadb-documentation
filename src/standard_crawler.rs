use std::collections::VecDeque;

use crate::{
    crawler::{read_and_write_content, Crawler},
    req::ScrapeClient,
    scrape::scrape_urls,
    starting_urls::standard_starting_urls,
    Result,
};

pub struct StandardCrawler {
    queue: VecDeque<String>,
    resume: bool,
}
impl StandardCrawler {
    pub fn new(resume: bool) -> Self {
        Self {
            resume,
            queue: standard_starting_urls(),
        }
    }
}

impl Crawler for StandardCrawler {
    fn next_url(&mut self) -> Option<String> {
        self.queue.pop_front()
    }
    fn process_url(&mut self, url: &str, client: &mut ScrapeClient) -> Result<()> {
        let Some(content) = read_and_write_content(client, self.resume, url) else {
            return Ok(());
        };
        if let Ok(text) = String::from_utf8(content) {
            let urls = scrape_urls(&text);
            self.queue.extend(urls);
        }
        Ok(())
    }
}
