extern crate rafy;
use rafy::*;

pub fn main() {
    let playlist = Playlist::<YoutubeDL>::new("https://www.youtube.com/playlist?list=PLB9cXA-RF0jMh3QutPBHmkyDj81DHwVSF").unwrap();
    for vid in playlist.videos {
        println!("URL {}", vid.url);
        vid.video().unwrap();
    }
}
