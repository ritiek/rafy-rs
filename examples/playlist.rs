extern crate rafy;
use rafy::*;
use std::env;

// Usage: cargo run --example playlist <playlist-url>

pub fn main() {

    let args: Vec<String> = env::args().collect();

    let playlist = Playlist::<YoutubeDL>::new(&args[1]).unwrap();
    for vid in playlist.videos {
        println!("URL {}", vid.url);
        vid.video().unwrap();
    }
}
