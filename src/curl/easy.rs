use std::{error, mem, slice, str};
use std::cell::RefCell;
use std::ffi::CString;
use std::rc::{Rc, Weak};
use curl_sys::*;
use libc::*;

use super::{Response, Time};

type Error = Box<error::Error>;

pub struct Easy {
  handle: *mut CURL,
  header_buffer: Rc<RefCell<String>>,
  body_buffer: Rc<RefCell<String>>,
  error_buffer: Vec<u8>,
}

impl Easy {
  pub fn new() -> Result<Easy, Error> {
    unsafe {
      match curl_easy_init().as_mut() {
        Some(handle) => {
          let header_buffer = Rc::new(RefCell::new(String::new()));
          curl_easy_setopt(handle, CURLOPT_HEADERFUNCTION, write_function as *mut c_void);
          curl_easy_setopt(handle, CURLOPT_HEADERDATA, Rc::downgrade(&header_buffer));

          let body_buffer = Rc::new(RefCell::new(String::new()));
          curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION, write_function as *mut c_void);
          curl_easy_setopt(handle, CURLOPT_WRITEDATA, Rc::downgrade(&body_buffer));

          let error_buffer = vec!(0; CURL_ERROR_SIZE);
          curl_easy_setopt(handle, CURLOPT_ERRORBUFFER, error_buffer.as_ptr());

          Ok(Easy {
            handle: handle,
            header_buffer: header_buffer,
            body_buffer: body_buffer,
            error_buffer: error_buffer,
          })
        },
        None => Err(From::from("Failed to create an easy handle")),
      }
    }
  }

  pub fn set_url(&self, url: &str) -> Result<(), Error> {
    let url = try!(CString::new(url));
    self.set_option(CURLOPT_URL, url.as_ptr())
  }

  pub fn perform(&self) -> Result<Response, Error> {
    self.get_result(unsafe { curl_easy_perform(self.handle) })
      .map(|_| Response {
        header: self.header_buffer.borrow().trim().to_string(),
        body: self.body_buffer.borrow().trim().to_string(),
      })
  }

  pub fn get_time(&self) -> Result<Time, Error> {
    Ok(Time {
      namelookup: try!(self.get_info(CURLINFO_NAMELOOKUP_TIME)),
      connect: try!(self.get_info(CURLINFO_CONNECT_TIME)),
      pretransfer: try!(self.get_info(CURLINFO_PRETRANSFER_TIME)),
      starttransfer: try!(self.get_info(CURLINFO_STARTTRANSFER_TIME)),
      total: try!(self.get_info(CURLINFO_TOTAL_TIME)),
    })
  }

  fn get_result(&self, code: CURLcode) -> Result<(), Error> {
    match code {
      CURLE_OK => Ok(()),
      _ => Err(From::from(
        str::from_utf8(&self.error_buffer).map(|s| s.trim_matches('\u{0}')).unwrap_or("Unknown error"))),
    }
  }

  fn set_option<T>(&self, option: CURLoption, value: T) -> Result<(), Error> {
    self.get_result(unsafe { curl_easy_setopt(self.handle, option, value) })
  }

  fn get_info<T>(&self, info: CURLINFO) -> Result<T, Error> {
    let mut result = unsafe { mem::zeroed() };
    self.get_result(unsafe { curl_easy_getinfo(self.handle, info, &mut result) })
      .map(|_| result)
  }
}

impl Drop for Easy {
  fn drop(&mut self) {
    unsafe {
      curl_easy_cleanup(self.handle)
    }
  }
}

unsafe extern fn write_function(ptr: *mut c_char, size: size_t, nitems: size_t, userdata: *mut c_void) -> size_t {
  let buffer = mem::transmute::<_, Weak<RefCell<String>>>(userdata);

  if let Some(buffer) = buffer.upgrade() {
    if let Ok(line) = str::from_utf8(slice::from_raw_parts(ptr as *mut _, size * nitems)) {
      buffer.borrow_mut().push_str(line);
    }
  }

  size * nitems
}
