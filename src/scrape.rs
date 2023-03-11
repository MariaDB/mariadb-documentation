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

    let url = suffix
        .trim_start_matches('/')
        .trim_start_matches("kb/")
        .trim_start_matches("mariadb.com/kb/")
        .trim_start_matches("https://mariadb.com/kb/")
        .trim();

    String::from("https://mariadb.com/kb/") + url
}

pub fn valid_url(url: &str) -> bool {
    debug_assert_eq!(url, &format_url(url));
    if IGNORED_URL_SEGMENTS
        .iter()
        .any(|segment| url.contains(segment) || url.ends_with(segment.trim_end_matches('/')))
    {
        return false;
    };
    !url.trim_start_matches("https://mariadb.com")
        .trim_start_matches('/')
        .trim_start_matches("kb")
        .trim_start_matches('/')
        .is_empty()
}
