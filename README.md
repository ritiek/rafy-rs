# rafy

Rust library to fetch YouTube content metadata. An attempt to mimic [pafy](https://github.com/mps-youtube/pafy) but in Rust.

## Installation

Put the below under `[dependencies]` section in your `Cargo.toml`:

```
rafy = { git = "https://github.com/ritiek/rafy-rs.git" }
```

## Usage Examples

```
extern crate rafy;

use rafy::Rafy;

fn main() {
    let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8");
    println!("{}", content.url);
    println!("{}", content.title);
    println!("{}", content.rating);
    println!("{}", content.viewcount);
    println!("{}", content.author);
    println!("{}", content.length);

    for stream in content.streams {
        println!("{}", stream.extension);
        println!("{}", stream.quality);
        println!("{}", stream.url);
    }

}
```

## Notes

The library does not use YouTube API keys at the moment, they can be used to fetch more information about the video. Check out the progress in [develop branch](https://github.com/ritiek/rafy-rs).

## License

`The MIT License`
