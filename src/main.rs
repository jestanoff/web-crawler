mod fetch_links;
mod utils;
mod types;

use reqwest::blocking::Client;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use fetch_links::fetch_links;
use utils::{normalize_url, show_loading_indicator, show_completion_message};
use chrono::Utc;

fn main() {
    println!("Please enter the URL to crawl:");
    let mut url = String::new();
    std::io::stdin()
        .read_line(&mut url)
        .expect("Failed to read input");
    let url = url.trim();
    let url = normalize_url(url);

    let client = Client::new();
    let mut all_links = Vec::new();

    // Fetch links from the main page
    let start_time = show_loading_indicator(&url);
    let links = fetch_links(&url, &client);
    show_completion_message(&url, start_time);
    all_links.extend(links.clone());

    // Fetch links from the first level of pages
    for link in links {
        let normalized_link = normalize_url(&link.url);
        let start_time = show_loading_indicator(&normalized_link);
        let sub_links = fetch_links(&normalized_link, &client);
        show_completion_message(&normalized_link, start_time);
        all_links.extend(sub_links);
    }

    let json_data = json!(all_links);
    let domain = url
      .trim_start_matches("http://")
      .trim_start_matches("https://")
      .trim_end_matches('/')
      .to_string();
    let start_time_str = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let file_name = format!("outputs/{}_{}.json", domain, start_time_str);
    let mut file = File::create(&file_name).expect("Failed to create file");
    file.write_all(json_data.to_string().as_bytes())
        .expect("Failed to write to file");

    println!("\nLinks have been saved to links.json");
}