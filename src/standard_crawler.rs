use crate::crawler::Crawler;

pub struct StandardCrawler {
    resume: bool,
}
impl StandardCrawler {
    pub fn new(resume: bool) -> Self {
        Self { resume }
    }
}

impl Crawler for StandardCrawler {
    fn ignore_existing_files(&self) -> bool {
        self.resume
    }
}
