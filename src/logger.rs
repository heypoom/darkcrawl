extern crate fern;
extern crate chrono;
extern crate log;

use fern::colors::{ColoredLevelConfig, Color};
use std::io;

pub fn setup() {
  let colors = ColoredLevelConfig::new()
    .info(Color::Blue);

  fern::Dispatch::new()
    .format(move |out, message, record| {
        out.finish(format_args!(
          "({}) [{}] {}",
          chrono::Local::now().format("%H:%M:%S"),
          colors.color(record.level()),
          message
        ))
    })
    .level(log::LevelFilter::Info)
    .chain(io::stdout())
    .chain(fern::log_file("output.log").unwrap())
    .apply()
    .unwrap();
}

