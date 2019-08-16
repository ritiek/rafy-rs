# yt_info

![Rust Toolchain](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![Crates.io](https://img.shields.io/crates/v/rafy.svg)](https://crates.io/crates/yt_info)
[![Docs.rs](https://docs.rs/rafy/badge.svg)](https://docs.rs/yt_info/)

Rust library to fetch YouTube content and retrieve metadata. Fork of [rafy-rs](https://github.com/ritiek/rafy-rs)

## Installation

Put the below in your `Cargo.toml`

> [dependencies]
>
> yt_info = "0.3.0"

## Usage Examples

```rust
use yt_info::VideoInfo;
use std::fs::File;
use std::io;

fn main() {
    let youtube_token = env!("YOUTUBE_TOKEN");

    let video = VideoInfo::new(youtube_token,"https://www.youtube.com/watch?v=C0DPdy98e4c").unwrap();
    let streams = video.streams;
    let stream = &streams[0];

    let filename = format!("{}-stream.{}",&video.title,&stream.extension);

    let mut file = File::create(filename).unwrap();
    let mut stream_reader = stream.get_reader().unwrap();

    io::copy(&mut stream_reader,&mut file).unwrap();
}
```

For more examples check out the [**Documentation**](https://docs.rs/yt_info/).

## Limitations

- This library won't be able to fetch `audiostreams` and `videostreams` for unpopular videos, because YouTube does not generate separate streams for unpopular videos. However, it will still be able to fetch normal `streams`.

- Since this library does not depend on [youtube-dl](https://github.com/rg3/youtube-dl), there are some more things (not mentioning here) that we'll be missing out.

## Running Tests

```
$ cargo test
```

## Contributing

All pull requests all welcome

## Thanks

The basic method of extracting streams was stolen from [rust-youtube-downloader](https://github.com/smoqadam/rust-youtube-downloader) by [smoqadam](https://github.com/smoqadam).

## License

`The MIT License`
