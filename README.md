# rafy [![Crates.io](https://img.shields.io/crates/v/rafy.svg)](https://crates.io/crates/rafy) [![Docs.rs](https://docs.rs/rafy/badge.svg)](https://docs.rs/crate/rafy/0.1.1)

Rust library to fetch YouTube content metadata. An attempt to mimic [pafy](https://github.com/mps-youtube/pafy) but in Rust.

## Installation

Put the below under `[dependencies]` section in your `Cargo.toml`:

```
rafy = "*"
```

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
    println!("{}", content.author);
    println!("{}", content.length);
    println!("{}", content.thumbdefault);

    println!("{}", content.likes);
    println!("{}", content.dislikes);
    println!("{}", content.commentcount);
    println!("{}", content.description);
    println!("{}", content.thumbmedium);
    println!("{}", content.thumbhigh);
    println!("{}", content.thumbstandard);
    println!("{}", content.thumbmaxres);

    let ref streams = content.streams;

    for stream in streams {
        println!("{}", stream.extension);
        println!("{}", stream.quality);
        println!("{}", stream.url);
    }
    
    streams[0].download();

}                                                                                                                                                   
```

## Thanks

The base code was adapted from [rust-youtube-downloader](https://github.com/smoqadam/rust-youtube-downloader) by [smoqadam](https://github.com/smoqadam), modified and further extended to suit the library accordingly.

## License

`The MIT License`
