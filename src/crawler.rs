use std::thread;
use std::sync::{Arc, Mutex, mpsc};

use reqwest::{Client, Proxy, Response};
use reqwest::header::ContentType;
use scraper::{Html, Selector};

use super::*;
use colored::*;

#[derive(Clone)]
pub struct Crawler {
  client: Client,
  success_urls: Vec<String>,
  failed_urls: Vec<String>
}

impl Crawler {
  pub fn new() -> Crawler {
    let tor_proxy = Proxy::http("http://localhost:8123").unwrap();

    logger::setup();

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
    let successes = self.success_urls.clone();
    let fails = self.failed_urls.clone();

    if !url.contains(".onion") {
      return Err(IsClearnet);
    }

    if successes.contains(&url) {
      return Err(AlreadyCrawled);
    }

    if fails.contains(&url) {
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
    // Perform the URL sanity check before proceeding
    if let Err(reason) = self.parse_url(url.to_string()) {
      warn!("Ignored {} because {}", url.dimmed(), reason.to_string().yellow());
      return
    }

    let c = Arc::new(Mutex::new(self.clone()));
    let url = String::from(url);

    // Spawn a new thread to handle
    let _ = thread::spawn(move || {
      let mut c = c.lock().unwrap();

      info!("Spawning a Thread to handle {}", url.cyan().bold().underline());

      // Fetch the resource via HTTP GET
      match c.client.get(&url).send() {
        Ok(res) => {
          if res.status().is_success() {
            info!("Retrieved {}. Parsing...", url);

            c.parse(&url, res);
          }
        },
        Err(err) => {
          info!("Network Error: {}", err);
          c.failed_urls.push(url.to_string());
        }
      }
    }).join();
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
      Err(err) => info!("{} is not a text file. ({})", url, err)
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

      if let Some(url) = link.value().attr("href") {
        info!("Link Found in <a>: {} ({:?})", &url.blue(), text);

        self.crawl(&url);
      } else {
        warn!("<a> does not contain href. Skipping...");
      }
    }
  }
}
