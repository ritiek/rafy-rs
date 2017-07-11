# rafy

[![Crates.io](https://img.shields.io/crates/v/rafy.svg)](https://crates.io/crates/rafy) [![Docs.rs](https://docs.rs/rafy/badge.svg)](https://docs.rs/rafy/) [![Build Status](https://travis-ci.org/ritiek/rafy-rs.svg?branch=master)](https://travis-ci.org/ritiek/rafy-rs)

Rust library to fetch YouTube content metadata. An attempt to mimic [pafy](https://github.com/mps-youtube/pafy) but in Rust.

## Installation

Put the below in your `Cargo.toml`

> [dependencies]
>
> rafy = "0.1"

## Usage Examples

```
extern crate rafy;

use rafy::Rafy;

fn main() {
    let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8");
    println!("{}", content.videoid);
    println!("{}", content.title);
    println!("{}", content.rating);
    println!("{}", content.viewcount);
}
```

For more examples check out the [**Documentation**](https://docs.rs/rafy/).

## Running Tests

`cargo test`

## Thanks

The base code was adapted from [rust-youtube-downloader](https://github.com/smoqadam/rust-youtube-downloader) by [smoqadam](https://github.com/smoqadam), modified and further extended to suit the library accordingly.

## License

`The MIT License`
