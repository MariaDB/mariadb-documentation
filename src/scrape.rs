pub const IGNORED_URL_SEGMENTS: &[&str] = &[
    "/+change_order/",
    "/add/",
    "/ask/",
    "/remove/",
    "/+history/",
    "/+translate/",
    "/+flag/",
    "/post/",
    "/+r/",
];

pub fn scrape_urls(html: &str) -> impl Iterator<Item = String> + '_ {
    let scraped_urls = scrape_raw_urls(html);
    filter_urls(scraped_urls)
}

fn scrape_raw_urls(html: &str) -> impl Iterator<Item = String> + '_ {
    let re = lazy_regex::regex!(r#"="(?:(?:https://)?mariadb.com)?/?kb/([^" ]*)""#);
    re.captures_iter(html).map(|cap| cap[1].to_owned())
}

pub fn filter_urls(urls: impl Iterator<Item = String>) -> impl Iterator<Item = String> {
    urls.map(format_url).filter(|url| valid_url(url))
}

pub fn format_url(url: impl AsRef<str>) -> String {
    let mut suffix = url.as_ref();
    for symbol in ['#', '?'] {
        if let Some(idx) = suffix.find(symbol) {
            suffix = &suffix[..idx];
        }
        debug_assert!(!suffix.contains(symbol));
    }
    String::from("https://mariadb.com/kb/") + trim_url(suffix)
}

pub fn valid_url(url: &str) -> bool {
    if IGNORED_URL_SEGMENTS
        .iter()
        .any(|segment| url.contains(segment) || url.ends_with(segment.trim_end_matches('/')))
    {
        return false;
    };
    if !url.starts_with("https://mariadb.com/kb") {
        return false;
    }
    !url.trim_start_matches("https://mariadb.com/kb")
        .contains('.')
}
pub fn trim_url(url: &str) -> &str {
    url.trim_start_matches("https://")
        .trim_start_matches('/')
        .trim_start_matches("mariadb.com")
        .trim_start_matches('/')
        .trim_start_matches("kb")
        .trim_matches('/')
}

#[cfg(test)]
mod tests {
    use super::*;
    fn kb(suffix: &str) -> String {
        format!("https://mariadb.com/kb/{suffix}")
    }
    #[test]
    fn test_trim() {
        assert_eq!(trim_url(&kb("en")), "en");
        assert_eq!(trim_url(&kb("en/")), "en");
        assert_eq!(trim_url(&kb("en/select")), "en/select");
    }
    #[test]
    fn test_valid() {
        let urls = ["en/select", "en", ""];
        for url in urls.map(format_url) {
            assert!(valid_url(&url), "{url}");
        }
    }
    #[test]
    fn test_invalid() {
        let full_urls = ["mariadb.com/kb/en/select", "https://test/kb/en/select"]
            .into_iter()
            .map(str::to_owned);
        let url_suffixes = ["en/remove/", "en/remove/meh"].into_iter().map(format_url);
        for url in full_urls.chain(url_suffixes) {
            assert!(!valid_url(&url), "{url}");
        }
    }
}
