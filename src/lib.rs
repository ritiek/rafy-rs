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

pub struct Rafy {
    pub videoid: String,
    pub title: String,
    pub rating: String,
    pub viewcount: u32,
    pub author: String,
    pub length: u32,
    pub thumbdefault: String,
    //pub duration: String,
    pub likes: u32,
    pub dislikes: u32,
    pub commentcount: u32,
    pub description: String,
    pub streams: Vec<Stream>,
    pub thumbmedium: String,
    pub thumbhigh: String,
    pub thumbstandard: String,
    pub thumbmaxres: String,
    //pub audiostreams: ,
    //pub allstreams: ,
}

pub struct Stream {
    pub extension: String,
    pub quality: String,
    pub url: String,
}


impl Stream {

    pub fn download(&self) {
        //download self.url
        let response = Rafy::send_request(&self.url);
        let file_size = Rafy::get_file_size(&response);
        let filename = "test.mp4";
        Self::write_file(response, &filename, file_size);
    }

    fn write_file(mut response: Response, title: &str, file_size: u64) {
        // initialize progressbar
        let mut pb = ProgressBar::new(file_size);
        pb.format("â•¢â–Œâ–Œâ–‘â•Ÿ");

        // Download and write to file
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

        let streams = Self::get_streams(&basic);

        Rafy {  videoid: videoid.to_string(),
                title: title.to_string(),
                rating: rating.to_string(),
                viewcount: viewcount.parse::<u32>().unwrap(),
                author: author.to_string(),
                length: length.parse::<u32>().unwrap(),
                thumbdefault: thumbdefault.to_string(),

                likes: likes.to_string()
                            .trim_matches('"')
                            .parse::<u32>()
                            .unwrap(),
                dislikes: dislikes.to_string()
                            .trim_matches('"')
                            .parse::<u32>()
                            .unwrap(),
                commentcount: commentcount.to_string()
                            .trim_matches('"')
                            .parse::<u32>()
                            .unwrap(),
                description: description.to_string(),
                thumbmedium: thumbmedium.to_string().trim_matches('"').to_string(),
                thumbhigh: thumbhigh.to_string().trim_matches('"').to_string(),
                thumbstandard: thumbstandard.to_string().trim_matches('"').to_string(),
                thumbmaxres: thumbmaxres.to_string().trim_matches('"').to_string(),

                streams: streams,
            }
    }

    fn get_streams(hq: &HashMap<String, String>) -> Vec<Stream> {
        let mut parsed_streams: Vec<Stream> = Vec::new();

        let streams: Vec<&str> = hq["url_encoded_fmt_stream_map"]
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
                        url: stream_url.to_string()
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
