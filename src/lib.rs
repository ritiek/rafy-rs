//! `rafy` is a simple-to-use library for downloading YouTube content and retrieving metadata.
//!
//! ## About
//!
//! `rafy` takes the YouTube URL of the video and parses it. It returns fields like title,
//! author, likes/dislikes and other information about the video. It can also be used to download video
//! and audio streams with selectable quality.
//!
//! ## Quick Example
//!
//! You need to add the line below in `[dependencies]` section in your `Cargo.toml`
//!
//! > rafy = "0.1"
//!
//! The following example shows how simple it is to use `rafy` to gather information about YouTube
//! videos.
//!
//! ```
//! extern crate rafy;
//! use rafy::Video;
//!
//! fn main() {
//!     let content = Video::new("https://www.youtube.com/watch?v=4I_NYya-WWg").unwrap();
//!     println!("{}", content.videoid);
//!     println!("{}", content.title);
//!     println!("{}", content.author);
//!     println!("{}", content.likes);
//!
//!     let streams = content.streams;
//!     for stream in streams {
//!        println!("{}", stream.extension);
//!        println!("{}", stream.url);
//!     }
//! }
//! ```
//!
//! You can also download YouTube videos by calling method `download()` on a `Stream` struct.
//!
//! ```
//! extern crate rafy;
//! use rafy::Video;
//!
//! fn main() {
//!     let content = Video::new("https://www.youtube.com/watch?v=AnRSXHQ2qyo").unwrap();
//!     let title = content.title;
//!     let streams = content.streams;
//!     // It is necessary to pass the filename to generate in download()
//!     streams[0].download(&title);
//!
//!     let audiostreams = content.audiostreams;
//!     audiostreams[0].download(&title);
//! }
//! ```
//!
//! The `youtube-dl` Python library can be used as backend:
//!
//! ```
//! extern crate rafy;
//! use rafy::Video;
//!
//! fn main() {
//!     let content = Video::new_with_youtube_dl("https://www.youtube.com/watch?v=AnRSXHQ2qyo").unwrap();
//!     let title = content.title;
//!     let streams = content.streams;
//!     // It is necessary to pass the filename to generate in download()
//!     streams[0].download(&title);
//!
//!     let audiostreams = content.audiostreams;
//!     audiostreams[0].download(&title);
//! }
//! ```
//!
//! ## License
//!
//! `rafy` is licensed under the MIT license. Please read the [LICENSE](LICENSE) file in
//! this repository for more information.


extern crate hyper;
extern crate hyper_native_tls;
extern crate pbr;
extern crate regex;
extern crate json;
extern crate cpython;
#[macro_use]
extern crate error_chain;

mod err;
mod playlist;
mod video;
mod stream;

pub use playlist::*;
pub use video::*;
pub use stream::*;
use err::*;

use std::str;
use std::collections::HashMap;
use hyper::client::response::Response;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::{ContentLength, Headers, ByteRangeSpec, Range};


fn send_request(url: &str) -> Result<Response> {
    let ssl = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    // Pass custom headers to fix speed throttle (issue #10)
    let mut header = Headers::new();
    header.set(Range::Bytes(vec![ByteRangeSpec::AllFrom(0)]));
    Ok(client.get(url).headers(header).send()?)
}

fn parse_url(query: &str) -> HashMap<String, String> {
    let url = format!("{}{}", "http://e.com?", query);
    let parsed_url = hyper::Url::parse(&url).unwrap();
    parsed_url.query_pairs()
            .into_owned()
            .collect()
}

// get file size from Content-Length header
fn get_file_size(response: &Response) -> u64 {
    let mut file_size = 0;
    match response.headers.get::<ContentLength>(){
        Some(length) => file_size = length.0,
        None => println!("Content-Length header missing"),
    };
    file_size
}
