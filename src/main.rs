extern crate curl_sys;
extern crate libc;

mod curl;

macro_rules! unwrap {
  ($expr:expr) => (match $expr {
    Ok(value) => value,
    Err(error) => panic!(error.to_string()),
  })
}

fn main() {
  let client = unwrap!(curl::Easy::new());
  unwrap!(client.set_url("http://example.com"));

  let response = unwrap!(client.perform());
  println!("{}", response.header);
  println!("{}", response.body);
  println!("{:?}", client.get_time());
}
