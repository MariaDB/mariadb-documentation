#[derive(Debug, Default)]
pub enum ScrapeMethod {
    #[default]
    Standard,
    RecentChanges,
    Pdf,
    PdfLangs,
}
