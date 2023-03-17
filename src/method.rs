#[derive(Debug, Default, Clone, Copy)]
pub enum ScrapeMethod {
    #[default]
    Standard,
    RecentChanges,
    Pdf,
    PdfLangs,
}

impl ScrapeMethod {
    pub fn is_complete_scrape(self) -> bool {
        matches!(self, Self::Standard | Self::RecentChanges)
    }
}
