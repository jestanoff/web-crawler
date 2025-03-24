# Web Crawler

This project is a simple web crawler that fetches the homepage of monzo.com and follows all the links on that page.

## Project Structure

```
web-crawler
├── src
│   ├── main.rs       # Entry point of the application
│   └── crawler.rs    # Implementation of the web crawler
├── Cargo.toml        # Configuration file for the Rust project
└── README.md         # Documentation for the project
```

## Dependencies

This project uses the following dependencies:

- `reqwest`: For making HTTP requests to fetch web pages.
- `select`: For parsing HTML and extracting links.

## Building the Project

To build the project, navigate to the project directory and run:

```
cargo build
```

## Running the Web Crawler

To run the web crawler, use the following command:

```
cargo run
```

This will start the crawler, which will fetch the homepage of monzo.com and follow all the links found on that page.

## Additional Information

This web crawler is a basic implementation and may not handle all edge cases. It is intended for educational purposes to demonstrate how to fetch and parse web pages in Rust.