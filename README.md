# rafy

[![Crates.io](https://img.shields.io/crates/v/rafy.svg)](https://crates.io/crates/rafy) [![Docs.rs](https://docs.rs/rafy/badge.svg)](https://docs.rs/rafy/) [![Build Status](https://travis-ci.org/ritiek/rafy-rs.svg?branch=master)](https://travis-ci.org/ritiek/rafy-rs)

Rust library to fetch YouTube content and retrieve metadata. An attempt to mimic [pafy](https://github.com/mps-youtube/pafy) but in Rust.

## Installation

Put the below in your `Cargo.toml`

> [dependencies]
>
> rafy = "0.2"

## Usage Examples

```rust
extern crate rafy;
use rafy::Rafy;

fn main() {
    let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8").unwrap();
    println!("{}", content.videoid);
    println!("{}", content.title);
    println!("{}", content.rating);
    println!("{}", content.viewcount);
}
```

For more examples check out the [**Documentation**](https://docs.rs/rafy/).

## Limitations

- This library won't be able to fetch `audiostreams` and `videostreams` for unpopular videos, because YouTube does not generate separate streams for unpopular videos. However, it will still be able to fetch normal `streams`.

- Since this library does not depend on [youtube-dl](https://github.com/rg3/youtube-dl), there are some more things (not mentioning here) that we'll be missing out.

## Running Tests

```
cargo test
```

## Contributing

- Rust is still new to me. If there is anything that can be improved, please open an issue or even better, send a PR! :smile:

- Documentation improvements are also most welcome!

## Thanks

The basic method of extracting streams was stolen from [rust-youtube-downloader](https://github.com/smoqadam/rust-youtube-downloader) by [smoqadam](https://github.com/smoqadam).

## License

`The MIT License`
