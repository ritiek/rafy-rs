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
}
```

## License

`The MIT License`
