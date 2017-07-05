extern crate hyper;
extern crate hyper_native_tls;
extern crate pbr;
extern crate regex;

use pbr::ProgressBar;
use std::{process,str};
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
        //Regex for youtube URLs.
        let url_regex = Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*").unwrap();
        let mut vid = url;

        if url_regex.is_match(vid) {
            let vid_split = url_regex.captures(vid).unwrap();
            vid = vid_split.get(1)
                    .unwrap()
                    .as_str();
        }

        let url_info = format!("https://youtube.com/get_video_info?video_id={}", vid);
        //println!("{}", url_info);

        let mut response = Self::send_request(&url_info);
        let mut response_str = String::new();
        response.read_to_string(&mut response_str).unwrap();
        let mut hq = Self::parse_url(&response_str);

        if hq["status"] != "ok" {
            println!("Video not found!");
            process::exit(1);
        }

        let title = &hq["title"];
        let rating = &hq["avg_rating"];
        let viewcount = &hq["view_count"];
        let author = &hq["author"];
        let length = &hq["length_seconds"];
        /*let streams: Vec<String> = hq["url_encoded_fmt_stream_map"]
                                    .split(',')
                                    .map(|s| Self::parse_url(s)["url"].to_string())
                                    .collect();*/

        let streams = Self::get_streams(&hq);

        //Self::download(hq);

        Rafy {
            url: url.to_string(),
            title: title.to_string(),
            rating: rating.to_string(),
            viewcount: viewcount.trim()
                        .parse::<u32>()
                        .unwrap(),
            author: author.to_string(),
            length: length.trim()
                        .parse::<u32>()
                        .unwrap(),
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

    fn download(hq: HashMap<String, String>) {
        // get streams
        let streams: Vec<&str> = hq["url_encoded_fmt_stream_map"]
            .split(',')
            .collect();

        // list of available qualities
        let mut qualities: HashMap<i32, (String, String)> = HashMap::new();
        for (i, url) in streams.iter().enumerate() {
            let quality = Self::parse_url(url);
            let extension = quality["type"]
                .split('/')
                .nth(1)
                .unwrap()
                .split(';')
                .next()
                .unwrap();
            qualities.insert(i as i32,
                             (quality["url"].to_string(), extension.to_owned()));
            println!("{}- {} {}",
                     i,
                     quality["quality"],
                     quality["type"]);
        }
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
        let u = format!("{}{}", "http://e.com?", query);
        let parsed_url = hyper::Url::parse(&u).unwrap();
        parsed_url.query_pairs().into_owned().collect()
    }
}
