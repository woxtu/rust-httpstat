use std::fmt;
use regex::Regex;

use super::{Body, Header, Time};

const RESET: &'static str = "\u{1b}[0m";
const BOLD: &'static str = "\u{1b}[1m";
const GREEN: &'static str = "\u{1b}[32m";
const CYAN: &'static str = "\u{1b}[36m";

impl fmt::Display for Header {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut buffer = Vec::new();

    for (index, line) in self.0.lines().enumerate() {
      match index {
        0 => {
          let re = Regex::new("(.+?)/(.*)").unwrap();
          buffer.push(String::new());
          buffer.push(re.replace(line, format!("{}$1{}/{}$2{}", GREEN, RESET, CYAN, RESET).as_str()));
        },
        _ => {
          let re = Regex::new("(.+?):(.*)").unwrap();
          buffer.push(re.replace(line, format!("$1:{}$2{}", CYAN, RESET).as_str()));
        },
      }
    }

    writeln!(f, "{}", buffer.join("\n"))
  }
}

impl fmt::Display for Body {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "{}Body{} stored in: {}", GREEN, RESET, self.0)
  }
}

fn format_a(x: f64) -> String {
    format!("{}{:^7}{}", CYAN, format!("{:.0}ms", x * 1000.0), RESET)
}

fn format_b(x: f64) -> String {
    format!("{}{:<7}{}", CYAN, format!("{:.0}ms", x * 1000.0), RESET)
}

impl fmt::Display for Time {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Time(is_https, ref time) = *self;

    if is_https {
      writeln!(f,
" {}DNS Lookup   TCP Connection   SSL Handshake   Server Processing   Content Transfer{}
[   {a0000}  |     {a0001}    |    {a0002}    |      {a0003}      |      {a0004}     ]
             |                |               |                   |                  |
    namelookup:{b0000}        |               |                   |                  |
                        connect:{b0001}       |                   |                  |
                                    pretransfer:{b0002}           |                  |
                                                      starttransfer:{b0003}          |
                                                                                 total:{b0004}",
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
    } else {
      writeln!(f,
" {}DNS Lookup   TCP Connection   Server Processing   Content Transfer{}
[   {a0000}  |     {a0001}    |      {a0003}      |      {a0004}     ]
             |                |                   |                  |
    namelookup:{b0000}        |                   |                  |
                        connect:{b0001}           |                  |
                                      starttransfer:{b0003}          |
                                                                 total:{b0004}",
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
  }
}
