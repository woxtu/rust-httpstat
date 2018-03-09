extern crate regex;
extern crate curl_sys;
extern crate libc;
extern crate clap;
extern crate rand;

pub mod httpstat;
pub mod curl;

pub use curl::easy::HttpVersion;

pub fn request(url: &str) -> Result<(curl::Response, curl::Time), String> {
    let client = curl::easy::Easy::new();
    match client {
        Err(e) => return Err(format!("{}", e)),
        _ => {},
    }
    let client = client.unwrap();

    match client.set_url(url) {
        Err(e) => return Err(format!("{}", e)),
        _ => {},
    }

    let response = client.perform();
    match response {
        Err(e) => return Err(format!("{}", e)),
        _ => {},
    }
    let response = response.unwrap();

    let time = client.get_time();
    match time {
        Err(e) => return Err(format!("{}", e)),
        _ => {}
    }
    let time = time.unwrap();

    Ok((response, time))
}
