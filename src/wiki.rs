use wikipedia::Wikipedia;
use std::{thread, time::Duration};

/// Fetch `qty` titles from Wikipedia with an exponential backoff timer starting at 2s.
pub fn get_wiki_titles(qty: u8, max_retries: u8) -> Vec<String> {
    let wiki_client = Wikipedia::<wikipedia::http::default::Client>::default();

    let mut retry: u8 = 0;
    let mut backoff: u32 = 2;
    let two: u32 = 2;

    while retry <= max_retries {
        let maybe_titles = wiki_client.random_count(qty);
        let titles = match maybe_titles {
            Ok(titles) => titles,
            Err(e) => {
                eprintln!("Error fetching wikipedia titles: {}", e.to_string());
                Vec::default()
            }
        };
        if titles.is_empty() {
            retry += 1;
            backoff = two.pow(retry as u32);
            eprintln!("Waiting {backoff} seconds before retrying...");
            thread::sleep(Duration::from_secs(backoff as u64));
            eprintln!("Retry {retry}/{max_retries}");
            continue;
        } else {
            return titles;
        }
    };
    panic!("Panic! Unable to fetch wikipedia titles. See stderr log for error details.");
}
