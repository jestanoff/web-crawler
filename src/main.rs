mod fetch_links;

use reqwest::blocking::Client;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use fetch_links::fetch_links;

#[derive(serde::Serialize, Clone)]
struct Link {
    id: String,
    timestamp: u64,
    url: String,
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