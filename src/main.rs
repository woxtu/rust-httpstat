extern crate clap;
extern crate httpstat;
extern crate rand;
extern crate regex;

use std::io::prelude::*;
use clap::App;

fn main() {
  let args = App::new("httpstat")
    .version(env!("CARGO_PKG_VERSION"))
    .about("curl statistics made simple")
    .args_from_usage(
      "<url>  'URL to work with'
      --http2 'use HTTP version 2'"
    )
    .get_matches();

  match httpstat::httpstat::app::run(&args) {
    Ok(()) => (),
    Err(error) => {
      let _ = write!(std::io::stderr(), "{}", error.to_string());
    },
  }
}
