extern crate hyper;
extern crate hyper_native_tls;
extern crate pbr;
extern crate regex;
extern crate serde_json;

use serde_json::{Value};
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
    pub url: String,
    pub title: String,
    pub rating: String,
    pub viewcount: u32,
    pub author: String,
    pub length: u32,
    pub thumb: String,
    //pub bigthumb: String,
    //pub duration: String,
    //pub likes: u32,
    //pub dislikes: u32,
    //pub description: String,
    pub streams: Vec<Stream>,
    //pub audiostreams: ,
    //pub allstreams: ,
}

pub struct Stream {
    pub extension: String,
    pub quality: String,
    pub url: String,
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
        println!("{}", api_info);

        let mut url_response = Self::send_request(&url_info);
        let mut url_response_str = String::new();
        url_response.read_to_string(&mut url_response_str).unwrap();
        let mut basic = Self::parse_url(&url_response_str);

        let mut api_response = Self::send_request(&api_info);
        let mut api_response_str = String::new();
        api_response.read_to_string(&mut api_response_str).unwrap();

        let parsed_json: Value = serde_json::from_str(&api_response_str).unwrap();
        println!("{}", parsed_json["etag"]);

        if basic["status"] != "ok" {
            println!("Video not found!");
            process::exit(1);
        }

        let title = &basic["title"];
        let rating = &basic["avg_rating"];
        let viewcount = &basic["view_count"];
        let author = &basic["author"];
        let length = &basic["length_seconds"];
        let thumb = &basic["thumbnail_url"];
        let streams = Self::get_streams(&basic);

        Rafy {  url: url.to_string(),
                title: title.to_string(),
                rating: rating.to_string(),
                viewcount: viewcount.trim()
                            .parse::<u32>()
                            .unwrap(),
                author: author.to_string(),
                length: length.trim()
                            .parse::<u32>()
                            .unwrap(),
                thumb: thumb.to_string(),
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

}
