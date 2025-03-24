use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::Link;

pub fn fetch_links(url: &str, client: &Client) -> Vec<Link> {
    let mut links = Vec::new();
    let response = client
        .get(url)
        .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Version/14.0 Safari/537.36")
        .send()
        .expect("Failed to fetch the URL");
    let body = response.text().expect("Failed to read response body");

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").expect("Failed to create selector");
    for (_, element) in document.select(&selector).enumerate() {
        if let Some(href) = element.value().attr("href") {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            let absolute_url = reqwest::Url::parse(url)
                .and_then(|base| base.join(href))
                .unwrap_or_else(|_| reqwest::Url::parse("http://example.com").unwrap());
            links.push(Link {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp,
                url: absolute_url.to_string(),
            });
        }
    }
    links
}
