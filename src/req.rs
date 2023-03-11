use std::error::Error;

use reqwest::blocking::{self, Response};

pub struct ScrapeClient {
    inner: blocking::Client,
}
pub struct ScrapeResult {
    pub content: Vec<u8>,
    pub file_extension: Option<String>,
    pub directed_url: String,
}
impl ScrapeClient {
    pub fn new() -> Self {
        Self {
            inner: blocking::Client::new(),
        }
    }
    pub fn get(&mut self, url: &str) -> Result<ScrapeResult, Box<dyn Error>> {
        log::info!("Requesting: {url}");
        let response = self.get_until_connect(url)?;
        let status = response.status();
        if status.is_server_error() {
            log::error!("Server Error: '{status}' for url {url}");
        } else if status.is_client_error() {
            log::warn!("Invalid Status: {status} for url {url}");
        }
        let response = response.error_for_status()?;
        let directed_url = response.url().to_string();
        if directed_url.contains("https://id.mariadb.com") {
            return Err("redirected to id".into());
        }
        let result = ScrapeResult {
            directed_url,
            file_extension: get_file_extension(&response).map(str::to_owned),
            content: response.bytes()?.to_vec(),
        };
        Ok(result)
    }
    fn get_until_connect(&mut self, url: &str) -> Result<Response, reqwest::Error> {
        loop {
            match self.inner.get(url).send() {
                Ok(response) => break Ok(response),
                Err(err) => {
                    if err.is_connect() || err.is_request() {
                        log::warn!("Connection Failed: '{url}'");
                        std::thread::sleep(std::time::Duration::from_secs(30));
                    } else {
                        log::error!("Unknown Error: '{err}'");
                        break Err(err);
                    }
                }
            }
        }
    }
}

fn get_file_extension(response: &Response) -> Option<&str> {
    let content_header = response.headers().get("content-type")?;
    let content_str = content_header.to_str().ok()?;
    let content_type = content_str.split(';').next()?;
    let extention = content_type.split_once('/')?.1;
    Some(extention)
}
