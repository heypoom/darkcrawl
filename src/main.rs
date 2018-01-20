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
      AlreadyCrawled {
        description("URL is already crawled")
        display("URL is already crawled")
      }

      // TODO: Store reason on why it had failed in the first place.
      PreviouslyFailed {
        description("Crawler had previously failed to retrieve this page")
        display("Crawler had previously failed to retrieve this page")
      }

      // TODO: Implement Relative URL Resolver
      IsRelative {
        description("Relative URL is unimplemented")
        display("Relative URL is unimplemented")
      }

      NonHTTP {
        description("Non-HTTP protocols are unsupported")
        display("Non-HTTP protocols are unsupported")
      }

      // TODO: Add flags to allow scraping clearnet URLs
      IsClearnet {
        description("Clearnet URLs are ignored")
        display("Clearnet URLs are ignored")
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
    if !url.contains(".onion") {
      return Err(IsClearnet);
    }

    if self.success_urls.contains(&url) {
      return Err(AlreadyCrawled);
    }

    if self.failed_urls.contains(&url) {
      return Err(PreviouslyFailed);
    }

    // TODO: Use a more precise check for relative urls
    if url.starts_with("..") {
      return Err(IsRelative);
    }

    // FIXME: Relative URLs does not start with http!
    if !url.starts_with("http") {
      return Err(NonHTTP);
    }

    Ok(url)
  }

  fn crawl(&mut self, url: &str) {
    if let Err(reason) = self.parse_url(url.to_string()) {
      println!("Ignored {} because {}", url, reason);
      return
    };

    println!("Crawling: {}", url);

    match self.client.get(url).send() {
      Ok(ref mut res) => {
        let body = res.text().unwrap();

        println!();

        if res.status().is_success() {
          self.success_urls.push(url.to_string());

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

