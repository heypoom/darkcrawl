extern crate select;
extern crate reqwest;
extern crate scraper;

use std::thread;
use std::sync::mpsc;
use reqwest::{Client, Proxy};
use scraper::{Html, Selector};

struct Crawler {
  client: Client,
  crawled: Vec<String>
}

impl Crawler {
  fn new() -> Crawler {
    let tor_proxy = Proxy::http("http://localhost:8123").unwrap();

    let client = Client::builder()
      .proxy(tor_proxy)
      .build()
      .unwrap();

    Crawler { client, crawled: vec![] }
  }

  fn crawl(&mut self, url: &str) {
    if self.crawled.contains(&url.to_string()) {
      println!("I already crawled {}! Ignoring...", url);
    } else if url.starts_with("..") {
      println!("Relative URL found: {}. Resolving...", url);
    } else if !url.starts_with("http") {
      println!("Non-HTTP URL found: {}. Resolving...", url);
    } else {
      println!("Crawling: {}", url);

      self.crawled.push(url.to_string());
      // println!("Crawled List: {:?}", self.crawled);

      match self.client.get(url).send() {
        Ok(ref mut res) => {
          let body = res.text().unwrap();

          if res.status().is_success() {
            println!("Successfully Crawled {}. Parsing Document...", url);
            self.parse(&body)
          }
        },
        Err(err) => println!("Network Error: {}", err)
      }
    }
  }

  fn parse(&mut self, body: &str) {
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

  let mut crawler = Crawler::new();
  crawler.crawl("http://es2adizg32j3kob5.onion");
}

