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
//! use rafy::Rafy;
//!
//! fn main() {
//!     let content = Rafy::new("https://www.youtube.com/watch?v=4I_NYya-WWg").unwrap();
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
//! use rafy::Rafy;
//!
//! fn main() {
//!     let content = Rafy::new("https://www.youtube.com/watch?v=AnRSXHQ2qyo").unwrap();
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

use pbr::ProgressBar;
use std::str;
use std::collections::HashMap;
use hyper::client::response::Response;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::{ContentLength, Headers, ByteRangeSpec, Range};
use std::io::Read;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use errors::{VideoNotFound, VideoUnavailable};
use std::error::Error;
use hyper::status::StatusCode;

pub mod errors;

/// Once you have created a Rafy object using `Rafy::new()`, several data attributes are available.
///
/// # Examples
///
/// ```
/// extern crate rafy;
/// use rafy::Rafy;
///
/// fn main() {
///     let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8").unwrap();
///     println!("{}", content.title);
///     println!("{}", content.viewcount);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Rafy {
    /// The 11-character video id
    pub videoid: String,
    /// The title of the video
    pub title: String,
    /// The viewcount of the video
    pub viewcount: u32,
    /// The author of the video
    pub author: String,
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
    /// The available streams (containing both video and audio)
    pub streams: Vec<Stream>,
    /// The available only-video streams
    pub videostreams: Vec<Stream>,
    /// The available only-audio streams
    pub audiostreams: Vec<Stream>,
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
///     let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8").unwrap();
///     for stream in content.streams {
///         println!("{}", stream.extension);
///         println!("{}", stream.url);
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Stream {
    /// The extension of the stream
    pub extension: String,
    /// The quality of the stream
    pub quality: String,
    /// The url of the stream
    pub url: String,
}

/// Create a `Vec<Stream>` object by calling `Rafy::new().streams` .
///
/// # Examples
///
/// ```
/// extern crate rafy;
/// use rafy::Rafy;
///
/// fn main() {
///     let content = Rafy::new("https://www.youtube.com/watch?v=DjMkfARvGE8").unwrap();
///     let streams = content.streams;
///     let ref stream = streams[0];
/// }
/// ```

impl Stream {
    /// Downloads the content stream from `Stream` object.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate rafy;
    /// use rafy::Rafy;
    ///
    /// fn main() {
    ///     let content = Rafy::new("https://www.youtube.com/watch?v=C0DPdy98e4c").unwrap();
    ///     let title = content.title;
    ///     let streams = content.streams;
    ///     let stream = &streams[0];
    ///     // It is necessary to pass the filename to generate in download()
    ///     stream.download(&title).unwrap();
    ///
    ///     let audiostreams = content.audiostreams;
    ///     let ref audiostream = audiostreams[0];
    ///     audiostream.download(&title).unwrap();
    ///
    ///     let videostreams = content.videostreams;
    ///     let ref videostream = videostreams[0];
    ///     videostream.download(&title).unwrap();
    /// }
    /// ```
    pub fn download(&self, title: &str) -> Result<(), Box<dyn Error>> {
        let response = Rafy::send_request(&self.url)?;
        let file_size = Rafy::get_file_size(&response)?;
        let file_name = format!("{}.{}", title, &self.extension);
        Self::write_file(response, &file_name, file_size);
        Ok(())
    }

    /// Return content reader from `Stream` object.
    ///
    /// # Examples
    ///
    /// ```
    /// use rafy::Rafy;
    /// use std::fs::File;
    /// use std::io;
    /// fn main() {
    ///     let video = Rafy::new("https://www.youtube.com/watch?v=C0DPdy98e4c").unwrap();
    ///     let streams = video.streams;
    ///     let stream = &streams[0];
    ///
    ///     let filename = format!("{}-stream.{}",&video.title,&stream.extension);
    ///
    ///     let mut file = File::create(filename).unwrap();
    ///     let mut stream_reader = stream.get_reader().unwrap();
    ///
    ///     io::copy(&mut stream_reader,&mut file).unwrap();
    /// }
    /// ```
    pub fn get_reader(&self) -> Result<impl Read, Box<dyn Error>> {
        let response = Rafy::send_request(&self.url)?;
        return Ok(response);
    }

    pub fn get_size(&self) -> Result<u64, Box<dyn Error>> {
        let response = Rafy::send_request(&self.url)?;
        let size = Rafy::get_file_size(&response)?;
        Ok(size)
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

    pub fn new(url: &str) -> Result<Rafy, Box<dyn std::error::Error>> {
        // API key to fetch content
        let key = "AIzaSyBbUAdvXsRpgaPQzBD7JFRCXlhW8U9uMJo";
        // Regex for youtube URLs
        let url_regex = Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*")?;
        let mut vid = url;

        if url_regex.is_match(vid) {
            let vid_split = url_regex.captures(vid).unwrap();
            vid = vid_split.get(1).unwrap().as_str();
        }

        let url_info = format!("https://youtube.com/get_video_info?video_id={}", vid);
        let api_info = format!("https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet,statistics&key={}", vid, key);

        let mut url_response = match Self::send_request(&url_info) {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let mut url_response_str = String::new();
        url_response.read_to_string(&mut url_response_str)?;
        let basic = Self::parse_url(&url_response_str);

        let mut api_response = match Self::send_request(&api_info) {
            Ok(response) => response,
            Err(e) => return Err(e),
        };
        let mut api_response_str = String::new();
        api_response.read_to_string(&mut api_response_str)?;

        let parsed_json = json::parse(&api_response_str)?;

        if basic["status"] != "ok" {
            return Err(Box::new(VideoNotFound::default()));
        }

        // dbg!(&api_info);
        // dbg!(&url_info);

        let videoid = &parsed_json["items"][0]["id"];
        let title = &parsed_json["items"][0]["snippet"]["title"];
        let viewcount = &parsed_json["items"][0]["statistics"]["viewCount"];
        let author = &parsed_json["items"][0]["snippet"]["channelTitle"];
        let thumbdefault = &parsed_json["items"][0]["snippet"]["thumbnails"]["default"];
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

        let (streams, videostreams, audiostreams) = Self::get_streams(&basic);

        Ok(Rafy {
            videoid: videoid.to_string(),
            title: title.to_string(),
            viewcount: viewcount.to_string().parse::<u32>()?,
            author: author.to_string(),
            thumbdefault: thumbdefault.to_string(),
            likes: likes.to_string().parse::<u32>()?,
            dislikes: dislikes.to_string().parse::<u32>()?,
            // Do not return error when comments are disabled
            commentcount: commentcount.to_string().parse::<u32>().unwrap_or(0),
            description: description.to_string(),
            thumbmedium: thumbmedium.to_string(),
            thumbhigh: thumbhigh.to_string(),
            thumbstandard: thumbstandard.to_string(),
            thumbmaxres: thumbmaxres.to_string(),
            published: published.to_string(),
            category: category.to_string().parse::<u32>()?,
            streams,
            videostreams,
            audiostreams,
        })
    }

    fn get_streams(basic: &HashMap<String, String>) -> (Vec<Stream>, Vec<Stream>, Vec<Stream>) {
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

            let parsed_stream = Stream {
                extension: extension.to_string(),
                quality: quality.to_string(),
                url: stream_url.to_string(),
            };


            parsed_streams.push(parsed_stream);
        }

        let mut parsed_videostreams: Vec<Stream> = Vec::new();
        let mut parsed_audiostreams: Vec<Stream> = Vec::new();

        if basic.contains_key("adaptive_fmts") {
            let streams: Vec<&str> = basic["adaptive_fmts"]
                .split(',')
                .collect();

            for url in streams.iter() {
                let parsed = Self::parse_url(url);
                let extension = &parsed["type"]
                    .split('/')
                    .nth(1)
                    .unwrap()
                    .split(';')
                    .next().unwrap();
                let stream_url = &parsed["url"];

                if parsed.contains_key("quality_label") {
                    let quality = &parsed["quality_label"];
                    let parsed_videostream = Stream {
                        extension: extension.to_string(),
                        quality: quality.to_string(),
                        url: stream_url.to_string(),
                    };

                    parsed_videostreams.push(parsed_videostream);
                } else {
                    let audio_extension = if extension == &"mp4" { "m4a" } else { extension };
                    let quality = &parsed["bitrate"];
                    let parsed_audiostream = Stream {
                        extension: audio_extension.to_string(),
                        quality: quality.to_string(),
                        url: stream_url.to_string(),
                    };

                    parsed_audiostreams.push(parsed_audiostream);
                }
            }
        }

        (parsed_streams, parsed_videostreams, parsed_audiostreams)
    }

    fn send_request(url: &str) -> Result<Response, Box<dyn Error>> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        // Pass custom headers to fix speed throttle (issue #10)
        let mut header = Headers::new();
        header.set(Range::Bytes(vec![ByteRangeSpec::AllFrom(0)]));
        let response = client.get(url).headers(header).send()?;
        Ok(response)
    }

    fn parse_url(query: &str) -> HashMap<String, String> {
        let url = format!("{}{}", "http://e.com?", query);
        let parsed_url = hyper::Url::parse(&url).unwrap();
        parsed_url.query_pairs()
            .into_owned()
            .collect()
    }

    // get file size from Content-Length header
    fn get_file_size(response: &Response) -> Result<u64, impl Error> {
        let mut file_size = 0;
        match response.headers.get::<ContentLength>() {
            Some(length) => file_size = length.0,
            None => return Err(VideoUnavailable::default()),
        };

        // Do not return size of 0 bytes
        /*if response.status != StatusCode::Ok {
            return Err(VideoUnavailable::default());
        }*/

        // Apparently youtube will still send correct file size, even with non-200 response code
        if file_size <= 0 {
            return Err(VideoUnavailable::default());
        }

        Ok(file_size)
    }
}

