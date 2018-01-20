extern crate darkcrawl;

use darkcrawl::Crawler;

fn main() {
  let mut crawler = Crawler::new();
  crawler.crawl("http://es2adizg32j3kob5.onion");
}

