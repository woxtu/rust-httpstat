extern crate curl_sys;

use std::ffi::CString;

fn main() {
  unsafe {
    let handle = curl_sys::curl_easy_init();
    let url = CString::new("http://example.com").unwrap();
    curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_URL, url.as_ptr());
    curl_sys::curl_easy_perform(handle);
    curl_sys::curl_easy_cleanup(handle);
  }
}
