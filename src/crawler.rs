use std::thread;
use std::sync::mpsc;

use reqwest::{Client, Proxy};
use reqwest::header::ContentType;
use scraper::{Html, Selector};

use super::*;

pub struct Crawler {
  client: Client,
  success_urls: Vec<String>,
  failed_urls: Vec<String>
}

impl Crawler {
  pub fn new() -> Crawler {
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

  pub fn crawl(&mut self, url: &str) {
    if let Err(reason) = self.parse_url(url.to_string()) {
      println!("Ignored {} because {}", url, reason);
      return
    };

    println!("Crawling: {}", url);

    match self.client.get(url).send() {
      Ok(mut res) => {
        let h = res.headers().clone();

        // TODO: Handle Based on Content Types
        //if let Some(t) = h.get::<ContentType>() {
          //println!("Content Type appears to be good.");
        //}

        match res.text() {
          Ok(body) => {
            if res.status().is_success() {
              self.success_urls.push(url.to_string());

              println!("Successfully Crawled {}. Parsing Document...", url);
              self.parse(&body)
            }
          },
          Err(err) => {
            println!("Failed to parse document: {}", err);
          }
        };
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
