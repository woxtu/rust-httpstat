extern crate curl_sys;
extern crate libc;

use std::ffi::CString;
use std::slice;
use libc::{c_char, size_t};

unsafe extern fn write_function(ptr: *mut c_char, size: size_t, nmemb: size_t, buffer: &mut String) -> size_t {
  buffer.push_str(&String::from_utf8_lossy(slice::from_raw_parts(ptr as *mut _, size * nmemb)));

  size * nmemb
}

fn main() {
  unsafe {
    let handle = curl_sys::curl_easy_init();
    let url = CString::new("http://example.com").unwrap();
    curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_URL, url.as_ptr());

    let mut header = String::new();
    curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_HEADERFUNCTION, write_function as *const c_char);
    curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_HEADERDATA, &mut header);

    let mut body = String::new();
    curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_WRITEFUNCTION, write_function as *const c_char);
    curl_sys::curl_easy_setopt(handle, curl_sys::CURLOPT_WRITEDATA, &mut body);

    curl_sys::curl_easy_perform(handle);

    println!("{}", header);
    println!("{}", body);

    curl_sys::curl_easy_cleanup(handle);
  }
}
