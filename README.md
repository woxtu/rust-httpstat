# rust-httpstat

![screenshot](screenshot.png)

curl statistics made simple. Rust implementation of [httpstat](https://github.com/reorx/httpstat).

## Installation

```bash
$ cargo install --git https://github.com/woxtu/rust-httpstat
```

## Usage
as command-line tool:
```
$ httpstat https://www.rust-lang.org/
```

as package:
```
extern crate httpstat;

fn main() {
    let url = "https://www.rust-lang.org/";
    match httpstat::request(url) {
        Ok((_resp, time)) => println!("{:?}", time),
        Err(e) => println!("fail httpstat request: {}", e),
    }
}
```


## License

Copyright (c) 2016 woxtu

Licensed under the MIT license.
