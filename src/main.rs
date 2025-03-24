use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(serde::Serialize, Clone)]
struct Link {
    id: String,
    timestamp: u64,
    url: String,
}

fn fetch_links(url: &str, client: &Client) -> Vec<Link> {
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

fn main() {
    println!("Please enter the URL to crawl:");
    let mut url = String::new();
    std::io::stdin()
        .read_line(&mut url)
        .expect("Failed to read input");
    let url = url.trim();

    let client = Client::new();
    let mut all_links = Vec::new();

    // Fetch links from the main page
    let links = fetch_links(url, &client);
    all_links.extend(links.clone());

    // Fetch links from the first level of pages
    for link in links {
        let sub_links = fetch_links(&link.url, &client);
        all_links.extend(sub_links);
    }

    let json_data = json!(all_links);
    let mut file = File::create("links.json").expect("Failed to create file");
    file.write_all(json_data.to_string().as_bytes())
        .expect("Failed to write to file");

    println!("Links have been saved to links.json");
}