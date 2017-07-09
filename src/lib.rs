//! `rafy` is a simple-to-use library for downloading YouTube content and retrieving metadata.
//!
//! ## About
//!
//! `rafy` takes the YouTube URL of the video and parses it. It returns fields like title,
//! author, likes/disklikes and other information about the video. It can also be used to download video
//! and audio streams with selectable quality.
//!
//! ## Quick Example
//!
//! You need to add the line below in `[dependencies]` section in your `Cargo.toml`
//!
//! ```
//! rafy = "*"
//! ```
//!
//! The following example shows how simple it is to use `rafy` to gather information about YouTube
//! videos.
//!
//! ```
//! extern crate rafy;
//! use rafy::Rafy;
//! 
//! fn main() {
//!     let content = Rafy::new("https://www.youtube.com/watch?v=4I_NYya-WWg");
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
//! use crate rafy::Rafy;
//!
//! fn main() {
//!     let content = Rafy::new("https://www.youtube.com/watch?v=4I_NYya-WWg");
//!     let stream = content.streams[0];
//!     stream.download();
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

use pbr::ProgressBar;
use std::{process, str};
use std::collections::HashMap;
use hyper::client::response::Response;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::ContentLength;
use std::io::Read;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

/// Once you have created a Rafy object using `Rafy::new()`, several data attributes are available.
///
/// # Examples
///
/// ```
/// extern crate rafy;
/// use rafy::Rafy;
///
/// fn main() {
///     let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8");
///     println!("{}", content.title);
///     println!("{}", content.viewcount);
/// }
/// ```

pub struct Rafy {
    /// The 11-character video id
    pub videoid: String,
    /// The title of the video
    pub title: String,
    /// The rating of the video (0-5)
    pub rating: String,
    /// The viewcount of the video
    pub viewcount: u32,
    /// The author of the video
    pub author: String,
    /// The duration of the streams in seconds
    pub length: u32,
    /// The url of the video’s thumbnail image
    pub thumbdefault: String,
    //pub duration: String,
    /// The number of likes received for the video
    pub likes: u32,
    /// The number of dislikes received for the video
    pub dislikes: u32,
    /// The commentcount of the video
    pub commentcount: u32,
    /// The video description text
    pub description: String,
    /// The available streams
    pub streams: Vec<Stream>,
    /// The url of the video’s medium size thumbnail image
    pub thumbmedium: String,
    /// The url of the video’s large size thumbnail image
    pub thumbhigh: String,
    /// The url of the video’s extra large thumbnail image
    pub thumbstandard: String, 
    /// The url of the video’s native thumbnail image
    pub thumbmaxres: String,
    /// The upload date of the video
    pub published: String,
    /// The category ID of the video
    pub category: u32,
    //pub audiostreams: ,
    //pub allstreams: ,
}

/// After creating a `Stream` struct, you can check its attributes or call methods on it.
///
/// # Examples
///
/// ```
/// extern crate rafy;
/// use rafy::Rafy;
///
/// fn main() {
///     let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8");
///     for stream in content.streams {
///         println!("{}", stream.extension);
///         println!("{}", stream.url);
///     }
/// }
/// ```

pub struct Stream {
    /// The extension of the stream
    pub extension: String,
    /// The quality of the stream
    pub quality: String,
    /// The url of the stream
    pub url: String,
    title: String,
}


impl Stream {

    /// Create a `Stream` object by calling `Rafy::new().streams[n]`.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate rafy;
    /// use rafy::Rafy;
    ///
    /// fn main() {
    ///     let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8");
    ///     let stream = contents.streams[0];
    /// }
    /// ```

    pub fn download(&self) {
        
        /// Downloads the stream from `Stream` object.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate rafy;
        /// use rafy::Rafy;
        ///
        /// fn main() {
        ///     let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8");
        ///     let stream = contents.stream[0];
        ///     stream.download();
        /// }
        /// ```
        
        let response = Rafy::send_request(&self.url);
        let file_size = Rafy::get_file_size(&response);
        let file_name = format!("{}.{}", &self.title, &self.extension);
        Self::write_file(response, &file_name, file_size);
    }

    fn write_file(mut response: Response, title: &str, file_size: u64) {
        let mut pb = ProgressBar::new(file_size);
        pb.format("╢▌▌░╟");

        let mut buf = [0; 128 * 1024];
        let mut file = File::create(title).unwrap();
        loop {
            match response.read(&mut buf) {
                Ok(len) => {
                    file.write_all(&buf[..len]).unwrap();
                    pb.add(len as u64);
                    if len == 0 {
                        break;
                    }
                    len
                }
                Err(why) => panic!("{}", why),
            };
        }
    }

}


impl Rafy {

    pub fn new(url: &str) -> Rafy {
        
        /// Create a Rafy object using the `Rafy::new()` function, giving YouTube URL as the argument.
        ///
        /// # Examples
        ///
        /// ```
        /// extern crate rafy;
        /// use rafy::Rafy;
        ///
        /// fn main() {
        ///     let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8");
        /// }
        /// ```
        
        // API key to fetch content
        let key = "AIzaSyDHTKjtUchUxUOzCtYW4V_h1zzcyd0P6c0";
        // Regex for youtube URLs
        let url_regex = Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*").unwrap();
        let mut vid = url;

        if url_regex.is_match(vid) {
            let vid_split = url_regex.captures(vid).unwrap();
            vid = vid_split.get(1)
                    .unwrap()
                    .as_str();
        }

        let url_info = format!("https://youtube.com/get_video_info?video_id={}", vid);
        let api_info = format!("https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet,statistics&key={}", vid, key);

        let mut url_response = Self::send_request(&url_info);
        let mut url_response_str = String::new();
        url_response.read_to_string(&mut url_response_str).unwrap();
        let basic = Self::parse_url(&url_response_str);

        let mut api_response = Self::send_request(&api_info);
        let mut api_response_str = String::new();
        api_response.read_to_string(&mut api_response_str).unwrap();

        let parsed_json = json::parse(&api_response_str).unwrap();

        if basic["status"] != "ok" {
            println!("Video not found!");
            process::exit(1);
        }

        //println!("{}", url_info);
        //println!("{}", api_info);

        let videoid = &basic["video_id"];
        let title = &basic["title"];
        let rating = &basic["avg_rating"];
        let viewcount = &basic["view_count"];
        let author = &basic["author"];
        let length = &basic["length_seconds"];
        let thumbdefault = &basic["thumbnail_url"];
        let likes = &parsed_json["items"][0]["statistics"]["likeCount"];
        let dislikes = &parsed_json["items"][0]["statistics"]["dislikeCount"];
        let commentcount = &parsed_json["items"][0]["statistics"]["commentCount"];
        let description = &parsed_json["items"][0]["snippet"]["description"];
        let thumbmedium = &parsed_json["items"][0]["snippet"]["thumbnails"]["medium"]["url"];
        let thumbhigh = &parsed_json["items"][0]["snippet"]["thumbnails"]["high"]["url"];
        let thumbstandard = &parsed_json["items"][0]["snippet"]["thumbnails"]["standard"]["url"];
        let thumbmaxres = &parsed_json["items"][0]["snippet"]["thumbnails"]["maxres"]["url"];
        let published = &parsed_json["items"][0]["snippet"]["publishedAt"];
        let category = &parsed_json["items"][0]["snippet"]["categoryId"];

        let streams = Self::get_streams(&basic);

        Rafy {  videoid: videoid.to_string(),
                title: title.to_string(),
                rating: rating.to_string(),
                viewcount: viewcount.parse::<u32>().unwrap(),
                author: author.to_string(),
                length: length.parse::<u32>().unwrap(),
                thumbdefault: thumbdefault.to_string(),
                likes: likes.to_string().parse::<u32>().unwrap(),
                dislikes: dislikes.to_string().parse::<u32>().unwrap(),
                commentcount: commentcount.to_string().parse::<u32>().unwrap(),
                description: description.to_string(),
                thumbmedium: thumbmedium.to_string(),
                thumbhigh: thumbhigh.to_string(),
                thumbstandard: thumbstandard.to_string(),
                thumbmaxres: thumbmaxres.to_string(),
                published: published.to_string(),
                category: category.to_string().parse::<u32>().unwrap(),
                streams: streams,
            }
    }

    fn get_streams(basic: &HashMap<String, String>) -> Vec<Stream> {
        let mut parsed_streams: Vec<Stream> = Vec::new();
        let streams: Vec<&str> = basic["url_encoded_fmt_stream_map"]
            .split(',')
            .collect();

        for url in streams.iter() {
            let parsed = Self::parse_url(url);
            let extension = &parsed["type"]
                .split('/')
                .nth(1)
                .unwrap()
                .split(';')
                .next()
                .unwrap();
            let quality = &parsed["quality"];
            let stream_url = &parsed["url"];
            let title = &basic["title"];

            let parsed_stream = Stream {
                        extension: extension.to_string(),
                        quality: quality.to_string(),
                        url: stream_url.to_string(),
                        title: title.to_string()
                    };

            parsed_streams.push(parsed_stream);
        }

        parsed_streams
    }

    fn send_request(url: &str) -> Response {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        client.get(url).send().unwrap_or_else(|e| {
            println!("Network request failed: {}", e);
            process::exit(1);
        })
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
}
