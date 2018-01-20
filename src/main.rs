extern crate select;
extern crate reqwest;
extern crate scraper;
#[macro_use] extern crate error_chain;

use std::thread;
use std::sync::mpsc;
use reqwest::{Client, Proxy};
use scraper::{Html, Selector};

mod errors {
  error_chain! {
    errors {
      AlreadyCrawled(url: String, is_success: bool) {
        description("URL is already crawled")
        display("URL '{}' is already crawled. Success: {}", url, is_success)
      }

      RelativeURL(url: String) {
        description("Relative URL is unimplemented")
        display("Relative URL is unimplemented. URL: {}", url)
      }

      NonHTTP(url: String) {
        description("Non HTTP Protocol is unimplemented")
        display("Relative URL is unimplemented. URL: {}", url)
      }
    }
  }
}

use errors::ErrorKind::*;

struct Crawler {
  client: Client,
  success_urls: Vec<String>,
  failed_urls: Vec<String>
}

impl Crawler {
  fn new() -> Crawler {
    let tor_proxy = Proxy::http("http://localhost:8123").unwrap();

    let client = Client::builder()
      .proxy(tor_proxy)
      .build()
      .unwrap();

    Crawler {
      client,
      success_urls: vec![],
      failed_urls: vec![]
    }
  }

  fn parse_url(&self, url: String) -> Result<String, errors::ErrorKind> {
    if self.success_urls.contains(&url) {
      return Err(AlreadyCrawled(url, true));
    }

    if self.failed_urls.contains(&url) {
      return Err(AlreadyCrawled(url, false));
    }

    if url.starts_with("..") {
      return Err(RelativeURL(url));
    }

    if !url.starts_with("http") {
      return Err(NonHTTP(url));
    }

    Ok(url)
  }

  fn crawl(&mut self, url: &str) {
    if let Err(err) = self.parse_url(url.to_string()) {
      println!("{}", err);
      return
    };

    println!("Crawling: {}", url);

    match self.client.get(url).send() {
      Ok(ref mut res) => {
        let body = res.text().unwrap();

        self.success_urls.push(url.to_string());

        if res.status().is_success() {
          println!("Successfully Crawled {}. Parsing Document...", url);
          self.parse(&body)
        }
      },
      Err(err) => {
        println!("Network Error: {}", err);
        self.failed_urls.push(url.to_string());
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

