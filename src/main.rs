extern crate curl_sys;
extern crate libc;

use std::env;

mod curl;

macro_rules! unwrap {
  ($expr:expr) => (match $expr {
    Ok(value) => value,
    Err(error) => panic!(error.to_string()),
  })
}

fn format_a(x: f64) -> String {
  format!("{:^7}", format!("{:.0}ms", x * 1000.0))
}

fn format_b(x: f64) -> String {
  format!("{:<7}", format!("{:.0}ms", x * 1000.0))
}

fn https_format(time: &curl::Time) -> String {
  format!(
"
  DNS Lookup   TCP Connection   SSL Handshake   Server Processing   Content Transfer
[   {a0000}  |     {a0001}    |    {a0002}    |      {a0003}      |      {a0004}     ]
             |                |               |                   |                  |
    namelookup:{b0000}        |               |                   |                  |
                        connect:{b0001}       |                   |                  |
                                    pretransfer:{b0002}           |                  |
                                                      starttransfer:{b0003}          |
                                                                                 total:{b0004}
",
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
  DNS Lookup   TCP Connection   Server Processing   Content Transfer
[   {a0000}  |     {a0001}    |      {a0003}      |      {a0004}     ]
             |                |                   |                  |
    namelookup:{b0000}        |                   |                  |
                        connect:{b0001}           |                  |
                                      starttransfer:{b0003}          |
                                                                 total:{b0004}
",
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
    println!("{}", response.header);

    let time = unwrap!(client.get_time());
    println!("{}", (if url.starts_with("https") { https_format } else { http_format })(&time));
  }
}
