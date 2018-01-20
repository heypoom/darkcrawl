extern crate select;
extern crate reqwest;
extern crate scraper;

use reqwest::{Client, Proxy};
use scraper::{Html, Selector};

fn get_client() -> Client {
    let tor_proxy = Proxy::http("http://localhost:8123").unwrap();

    Client::builder()
        .proxy(tor_proxy)
        .build()
        .unwrap()
}

fn main() {
    println!("Initializing...");
    let client = get_client();

    println!("Sending Request...");

    // res.status().is_success()
    let mut res = client.get("http://es2adizg32j3kob5.onion").send().unwrap();
    let body = res.text().unwrap();

    println!("Parsing Document...");

    let document = Html::parse_document(&body);
    let links = Selector::parse("a").unwrap();

    for link in document.select(&links) {
        let text: Vec<_> = link.text().collect();
        let href = link.value().attr("href").unwrap();
        println!("Link Found: {} ({:?})", href, text);
    }
}

