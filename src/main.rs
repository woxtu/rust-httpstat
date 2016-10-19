extern crate clap;
extern crate curl;
extern crate rand;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::Write;
use clap::App;
use rand::Rng;
use regex::Regex;

macro_rules! unwrap {
  ($expr:expr) => (match $expr {
    Ok(value) => value,
    Err(error) => panic!(error.to_string()),
  })
}

const RESET: &'static str = "\u{1b}[0m";
const BOLD: &'static str = "\u{1b}[1m";
const GREEN: &'static str = "\u{1b}[32m";
const CYAN: &'static str = "\u{1b}[36m";

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
  let args = App::new("httpstat")
    .version(env!("CARGO_PKG_VERSION"))
    .about("curl statistics made simple")
    .arg_from_usage("<url> 'URL to work with'")
    .get_matches();

  let url = args.value_of("url").unwrap();

  let client = unwrap!(curl::Easy::new());
  unwrap!(client.set_url(url));

  let response = unwrap!(client.perform());

  for (index, line) in response.header.lines().enumerate() {
    match index {
      0 => {
        let re = Regex::new("(.+?)/(.*)").unwrap();
        println!("");
        println!("{}", re.replace(line, format!("{}$1{}/{}$2{}", GREEN, RESET, CYAN, RESET).as_str()));
      },
        _ => {
          let re = Regex::new("(.+?):(.*)").unwrap();
          println!("{}", re.replace(line, format!("$1:{}$2{}", CYAN, RESET).as_str()));
        },
    }
  }

  let mut tempfile_path = env::temp_dir();
  tempfile_path.set_file_name(rand::thread_rng().gen_ascii_chars().take(20).collect::<String>());
  let mut tempfile = unwrap!(File::create(&tempfile_path));
  unwrap!(tempfile.write_all(response.body.as_bytes()));
  println!("");
  println!("{}Body{} stored in: {}", GREEN, RESET, tempfile_path.to_string_lossy());

  let time = unwrap!(client.get_time());
  println!("{}", (if url.starts_with("https") { https_format } else { http_format })(&time));
}
