use std::sync::{mpsc::channel, Arc};
use std::thread;
use std::time::{Duration, Instant};
use serde::Serialize;
use ureq;
use chrono::{Utc, DateTime};

#[derive(Serialize, Debug)]
struct WebsiteStatus {
    url: String,
    status: Result<u16, String>,
    response_time: Duration,
    timestamp: DateTime<Utc>,
}

fn check_website(url: &str, timeout: Duration, retries: u32) -> WebsiteStatus {
    let mut attempts = 0;
    let start_time = Instant::now();
    let timestamp = Utc::now();

    while attempts <= retries {
        let response = ureq::get(url).timeout(timeout).call();

        match response {
            Ok(res) => {
                return WebsiteStatus {
                    url: url.to_string(),
                    status: Ok(res.status()),
                    response_time: start_time.elapsed(),
                    timestamp,
                };
            }
            Err(_) if attempts < retries => attempts += 1,
            Err(err) => {
                return WebsiteStatus {
                    url: url.to_string(),
                    status: Err(err.to_string()),
                    response_time: start_time.elapsed(),
                    timestamp,
                };
            }
        }
    }

    WebsiteStatus {
        url: url.to_string(),
        status: Err("Max retries reached".to_string()),
        response_time: start_time.elapsed(),
        timestamp,
    }
}

fn monitor_websites(
    urls: Arc<Vec<String>>,
    num_threads: usize,
    timeout: Duration,
    retries: u32,
    tx: std::sync::mpsc::Sender<WebsiteStatus>,
) {
    let mut handles = Vec::new();

    for i in 0..num_threads {
        let urls = Arc::clone(&urls);
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            for (index, url) in urls.iter().enumerate() {
                if index % num_threads == i {
                    let status = check_website(url, timeout, retries);
                    tx.send(status).unwrap();
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn periodic_monitoring(
    urls: Vec<String>,
    num_threads: usize,
    timeout: Duration,
    retries: u32,
    interval: Duration,
) {
    let urls = Arc::new(urls);

    loop {
        let (tx, rx) = channel();

        thread::spawn({
            let urls = Arc::clone(&urls);
            move || {
                monitor_websites(urls, num_threads, timeout, retries, tx);
            }
        });

        for status in rx {
            println!("{:?}", status);
        }

        thread::sleep(interval);
    }
}

fn main() {
    let urls = vec![
        "https://www.google.com".to_string(),
        "https://www.youtube.com".to_string(),
        "https://www.facebook.com".to_string(),
        "https://www.twitter.com".to_string(),
        "https://www.instagram.com".to_string(),
        "https://www.linkedin.com".to_string(),
        "https://www.reddit.com".to_string(),
        "https://www.tiktok.com".to_string(),
        "https://www.snapchat.com".to_string(),
        "https://www.whatsapp.com".to_string(),
        "https://www.pinterest.com".to_string(),
        "https://www.tumblr.com".to_string(),
        "https://www.twitch.tv".to_string(),
        "https://www.medium.com".to_string(),
        "https://www.disney.com".to_string(),
        "https://www.coca-cola.com".to_string(),
        "https://www.pepsi.com".to_string(),
        "https://www.sprite.com".to_string(),
        "https://www.drpepper.com".to_string(),
        "https://www.fanta.com".to_string(),
        "https://www.microsoft.com".to_string(),
        "https://www.apple.com".to_string(),
        "https://www.netflix.com".to_string(),
        "https://www.spotify.com".to_string(),
        "https://www.amazon.com".to_string(),
        "https://www.ebay.com".to_string(),
        "https://www.walmart.com".to_string(),
        "https://www.target.com".to_string(),
        "https://www.adobe.com".to_string(),
        "https://www.nasa.gov".to_string(),
        "https://www.tesla.com".to_string(),
        "https://www.weather.com".to_string(),
        "https://www.tripadvisor.com".to_string(),
        "https://www.airbnb.com".to_string(),
        "https://www.booking.com".to_string(),
        "https://www.wikipedia.org".to_string(),
        "https://themousepadcompany.com".to_string(),
        "https://www.cnn.com".to_string(),
        "https://www.crazygames.com".to_string(),
        "https://www.nytimes.com".to_string(),
        "https://www.roblox.com".to_string(),
        "https://www.riotgames.com".to_string(),
        "https://www.forever21.com".to_string(),
        "https://www.blizzard.com".to_string(),
    ];

    let num_threads = 8;
    let timeout = Duration::from_secs(5);
    let retries = 3;
    let interval = Duration::from_secs(60);

    periodic_monitoring(urls, num_threads, timeout, retries, interval);
}
