use std::thread;
use std::sync::mpsc;

use reqwest::{Client, Proxy, Response};
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
    // Perform URL check before proceeding
    if let Err(reason) = self.parse_url(url.to_string()) {
      println!("Ignored {} because {}", url, reason);
      return
    }

    println!("Crawling: {}", url);

    // Fetch the resource via HTTP GET
    match self.client.get(url).send() {
      Ok(res) => {
        if res.status().is_success() {
          println!("Retrieved {}. Parsing...", url);

          self.parse(url, res);
        }
      },
      Err(err) => {
        println!("Network Error: {}", err);
        self.failed_urls.push(url.to_string());
      }
    }
  }

  // TODO: Check the Content-Type Header Before Parsing!
  fn parse(&mut self, url: &str, mut res: Response) {
    match res.text() {
      Ok(body) => {
        // Append the URL to the success list
        self.success_urls.push(url.to_string());

        // Write the result to file
        self.write_file(url, &body);

        // If it is a HTML File, parse them.
        self.parse_html(&body);
      },
      Err(err) => println!("{} is not a text file. ({})", url, err)
    }
  }

  // TODO: Write file to disk.
  fn write_file(&self, url: &str, content: &str) {

  }

  // Retrieve the URLs and crawl them
  fn parse_html(&mut self, body: &str) {
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
