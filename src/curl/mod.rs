extern crate curl_sys;
extern crate libc;

pub mod easy;

pub struct Response {
  pub header: String,
  pub body: String,
}

#[derive(Debug)]
pub struct Time {
  pub namelookup: f64,
  pub connect: f64,
  pub pretransfer: f64,
  pub starttransfer: f64,
  pub total: f64,
}

pub use self::easy::Easy;
