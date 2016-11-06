use curl;

pub mod app;

mod format;

pub struct Header(String);
pub struct Body(String);
pub struct Time(bool, curl::Time);
