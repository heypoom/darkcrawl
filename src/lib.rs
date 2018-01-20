extern crate select;
extern crate reqwest;
extern crate scraper;
extern crate chrono;
extern crate fern;
extern crate colored;
extern crate crossbeam;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

pub mod logger;
pub mod crawler;
pub mod errors;

pub use crawler::*;
pub use errors::ErrorKind::*;

