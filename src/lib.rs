extern crate select;
extern crate reqwest;
extern crate scraper;

#[macro_use]
extern crate error_chain;

pub mod crawler;
pub mod errors;

pub use crawler::Crawler;
pub use errors::ErrorKind::*;

