extern crate select;
extern crate reqwest;
extern crate scraper;

use std::thread;
use std::sync::mpsc;
use reqwest::{Client, Proxy};
use scraper::{Html, Selector};

struct Crawler {
  client: Client
}

impl Crawler {
  fn new() -> Crawler {
    let tor_proxy = Proxy::http("http://localhost:8123").unwrap();

    let client = Client::builder()
      .proxy(tor_proxy)
      .build()
      .unwrap();

    Crawler { client }
  }

  fn crawl(&self, url: &str) {
    println!("Crawling {}", url);

    match self.client.get(url).send() {
      Ok(ref mut res) => {
        let body = res.text().unwrap();

        if res.status().is_success() {
          self.parse(&body)
        }
      },
      Err(err) => println!("Network Error: {}", err)
    }
  }

  fn parse(&self, body: &str) {
    println!("Parsing Document...");

    let document = Html::parse_document(&body);
    let links = Selector::parse("a").unwrap();

    for link in document.select(&links) {
      let text: Vec<_> = link.text().collect();
      let url = link.value().attr("href").unwrap();
      println!("Link Found: {} ({:?})", &url, text);

      self.crawl(&url);
    }
  }
}

fn main() {
  println!("Initializing...");

  let crawler = Crawler::new();
  crawler.crawl("http://es2adizg32j3kob5.onion");
}

