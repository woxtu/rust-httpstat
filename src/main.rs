extern crate curl_sys;
extern crate libc;
extern crate regex;

use std::env;
use regex::Regex;

mod curl;

const RESET: &'static str = "\u{1b}[0m";
const BOLD: &'static str = "\u{1b}[1m";
const GREEN: &'static str = "\u{1b}[32m";
const CYAN: &'static str = "\u{1b}[36m";

macro_rules! unwrap {
  ($expr:expr) => (match $expr {
    Ok(value) => value,
    Err(error) => panic!(error.to_string()),
  })
}

fn format_a(x: f64) -> String {
  format!("{}{:^7}{}", CYAN, format!("{:.0}ms", x * 1000.0), RESET)
}

fn format_b(x: f64) -> String {
  format!("{}{:<7}{}", CYAN, format!("{:.0}ms", x * 1000.0), RESET)
}

fn https_format(time: &curl::Time) -> String {
  format!(
"
  {}DNS Lookup   TCP Connection   SSL Handshake   Server Processing   Content Transfer{}
[   {a0000}  |     {a0001}    |    {a0002}    |      {a0003}      |      {a0004}     ]
             |                |               |                   |                  |
    namelookup:{b0000}        |               |                   |                  |
                        connect:{b0001}       |                   |                  |
                                    pretransfer:{b0002}           |                  |
                                                      starttransfer:{b0003}          |
                                                                                 total:{b0004}
",
    BOLD, RESET,
    a0000 = format_a(time.namelookup),
    a0001 = format_a(time.connect - time.namelookup),
    a0002 = format_a(time.pretransfer - time.connect),
    a0003 = format_a(time.starttransfer - time.pretransfer),
    a0004 = format_a(time.total - time.starttransfer),
    b0000 = format_b(time.namelookup),
    b0001 = format_b(time.connect),
    b0002 = format_b(time.pretransfer),
    b0003 = format_b(time.starttransfer),
    b0004 = format_b(time.total))
}

fn http_format(time: &curl::Time) -> String {
  format!(
"
  {}DNS Lookup   TCP Connection   Server Processing   Content Transfer{}
[   {a0000}  |     {a0001}    |      {a0003}      |      {a0004}     ]
             |                |                   |                  |
    namelookup:{b0000}        |                   |                  |
                        connect:{b0001}           |                  |
                                      starttransfer:{b0003}          |
                                                                 total:{b0004}
",
    BOLD, RESET,
    a0000 = format_a(time.namelookup),
    a0001 = format_a(time.connect - time.namelookup),
    a0003 = format_a(time.starttransfer - time.pretransfer),
    a0004 = format_a(time.total - time.starttransfer),
    b0000 = format_b(time.namelookup),
    b0001 = format_b(time.connect),
    b0003 = format_b(time.starttransfer),
    b0004 = format_b(time.total))
}

fn main() {
  if let Some(url) = env::args().nth(1) {
    let client = unwrap!(curl::Easy::new());
    unwrap!(client.set_url("http://example.com"));

    println!("");

    let response = unwrap!(client.perform());

    for (index, line) in response.header.lines().enumerate() {
      match index {
        0 => {
          let re = Regex::new("(.+?)/(.*)").unwrap();
          println!("{}", re.replace(line, format!("{}$1{}/{}$2{}", GREEN, RESET, CYAN, RESET).as_str()));
        },
        _ => {
          let re = Regex::new("(.+?):(.*)").unwrap();
          println!("{}", re.replace(line, format!("$1:{}$2{}", CYAN, RESET).as_str()));
        },
      }
    }

    let time = unwrap!(client.get_time());
    println!("{}", (if url.starts_with("https") { https_format } else { http_format })(&time));
  }
}
