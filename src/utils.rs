use std::io::Write;
use std::time::{Duration, Instant};
use std::thread::sleep;

pub fn normalize_url(url: &str) -> String {
    let mut normalized = if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    };
    if !normalized.ends_with('/') {
        normalized.push('/');
    }
    normalized
}

pub fn show_loading_indicator(url: &str) -> Instant {
    let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let terminal_width = 160;
    let start_time = Instant::now();
    for frame in frames.iter() {
        let padded_url = format!("{:<width$}", url, width = terminal_width - 2);
        print!("\r{} {}", frame, padded_url);
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100));
    }
    start_time
}

pub fn show_completion_message(url: &str, start_time: Instant) {
    let elapsed = start_time.elapsed().as_secs_f64();
    let terminal_width = 160;
    let padded_url = format!("{:<width$}", url, width = terminal_width - 2);
    println!("\r✓ {} {:.3}s", padded_url, elapsed);
}
