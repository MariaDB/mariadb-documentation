#[derive(Debug, Default, Clone, Copy)]
pub enum ScrapeMethod {
    #[default]
    Standard,
    RecentChanges,
    Pdf,
    PdfLangs,
}
