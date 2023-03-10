use crate::crawler::Crawler;
use std::collections::HashSet;

pub struct StandardCrawler {
    completed: HashSet<String>,
    resume: bool,
    count: usize,
}
impl StandardCrawler {
    pub fn new(resume: bool) -> Self {
        Self {
            completed: HashSet::new(),
            resume,
            count: 0,
        }
    }
}

impl Crawler for StandardCrawler {
    fn is_valid_url(&mut self, url: &str) -> bool {
        if !self.completed.insert(url.to_owned()) {
            return false;
        }
        self.count += 1;
        if self.count % 1000 == 0 {
            log::info!("Processed {} Files", self.count);
        }
        true
    }
    fn ignore_existing_files(&self) -> bool {
        self.resume
    }
}
