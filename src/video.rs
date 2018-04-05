use err::*;
use stream::Stream;


use json;
use std::str;
use std::collections::HashMap;
use std::io::Read;
use regex::Regex;
use cpython::{Python, PyDict, ObjectProtocol, PyList};
use std::marker::PhantomData;


/// Once you have created a Video object using `Video::new()`, several data attributes are available.
///
/// # Examples
///
/// ```
/// extern crate rafy;
/// use rafy::Video;
///
/// fn main() {
///     let content = Video::new("https://www.youtube.com/watch?v=DjMkfARvGE8").unwrap();
///     println!("{}", content.title);
///     println!("{}", content.viewcount);
/// }
/// ```


#[derive(Debug, Clone)]
pub struct Video<B: Backend> {
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
    _marker: PhantomData<B>,
}

impl<B: Backend> Video<B> {
    pub fn new(url: &str) -> Result<Video<B>> {
        B::new_video(url)
    }
}


//////////////
// Backends //
//////////////

pub trait Backend: Sized {
    /// Create a Video object using the `Video::new()` function, giving YouTube URL as the argument.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate rafy;
    /// use rafy::Video;
    ///
    /// fn main() {
    ///     let content = Video::new("https://www.youtube.com/watch?v=DjMkfARvGE8");
    /// }
    /// ```
    fn new_video(url: &str) -> Result<Video<Self>>;
}

pub struct Internal {}
impl Backend for Internal {
    fn new_video(url: &str) -> Result<Video<Internal>> {
        // API key to fetch content
        let key = "AIzaSyDHTKjtUchUxUOzCtYW4V_h1zzcyd0P6c0";
        // Regex for youtube URLs
        let url_regex = Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*").unwrap();
        // let mut vid = url;

        let vid = if url_regex.is_match(url) {
            let vid_split = url_regex.captures(url).unwrap();
            vid_split.get(1).unwrap().as_str()
        } else {
            url
        };

        let url_info = format!("https://youtube.com/get_video_info?video_id={}", vid);
        let api_info = format!("https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet,statistics&key={}", vid, key);

        let mut url_response = match ::send_request(&url_info) {
            Ok(response) => response,
            Err(e) => bail!(Error::with_chain(e, ErrorKind::NetworkRequestFailed{})),
        };
        let mut url_response_str = String::new();
        url_response.read_to_string(&mut url_response_str)?;
        let basic = ::parse_url(&url_response_str);

        let mut api_response = match ::send_request(&api_info) {
            Ok(response) => response,
            Err(e) => bail!(Error::with_chain(e, ErrorKind::NetworkRequestFailed{})),
        };
        let mut api_response_str = String::new();
        api_response.read_to_string(&mut api_response_str)?;

        let parsed_json = json::parse(&api_response_str)?;

        if basic["status"] != "ok" {
            bail!(ErrorKind::VideoNotFound)
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

        let (streams, videostreams, audiostreams) = get_streams(&basic);

        Ok(Video {  videoid: videoid.to_string(),
                title: title.to_string(),
                rating: rating.to_string(),
                viewcount: viewcount.parse::<u32>()?,
                author: author.to_string(),
                length: length.parse::<u32>()?,
                thumbdefault: thumbdefault.to_string(),
                likes: likes.to_string().parse::<u32>()?,
                dislikes: dislikes.to_string().parse::<u32>()?,
                commentcount: commentcount.to_string().parse::<u32>()?,
                description: description.to_string(),
                thumbmedium: thumbmedium.to_string(),
                thumbhigh: thumbhigh.to_string(),
                thumbstandard: thumbstandard.to_string(),
                thumbmaxres: thumbmaxres.to_string(),
                published: published.to_string(),
                category: category.to_string().parse::<u32>()?,
                streams: streams,
                videostreams: videostreams,
                audiostreams: audiostreams,
                _marker: PhantomData,
            })
    }

}

pub struct YoutubeDL {}
impl Backend for YoutubeDL {
    fn new_video(url: &str) -> Result<Video<YoutubeDL>> {
        let url_regex = Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*").unwrap();

        let videoid = if url_regex.is_match(url) {
            let vid_split = url_regex.captures(url).unwrap();
            vid_split.get(1).unwrap().as_str()
        } else {
            url
        };

        let gil = Python::acquire_gil();
        let py = gil.python();
        let youtube_dl = py.import("youtube_dl")?;
        // In pafy, these options are sent in a python dict::
        //  def_ydl_opts = {'quiet': True, 'prefer_insecure': True, 'no_warnings': True}
        let ydl_info = {
            let ydl = {
                let kwargs = PyDict::new(py);
                kwargs.set_item(py, "quiet", py.True())?;
                kwargs.set_item(py, "prefer_insecure", py.True())?;
                kwargs.set_item(py, "no_warnings", py.True())?;
                youtube_dl.get(py, "YoutubeDL")?.call(py, (kwargs,), None)?
            };
            let kwargs = PyDict::new(py);
            kwargs.set_item(py, "download", py.False())?;
            ydl.call_method(py, "extract_info", (videoid,), Some(&kwargs))?
                .cast_into::<PyDict>(py).unwrap()
        };
        let (allstreams, audiostreams, videostreams) = get_streams_with_youtube_dl(py, &ydl_info)?;

        Ok(Video {
            videoid: videoid.to_string(), // TODO is this right?
            title: get_string(py, &ydl_info, "title")?,
            rating: format!("{}", get_u32(py, &ydl_info, "average_rating")?),
            viewcount: get_u32(py, &ydl_info, "view_count")?,
            author: get_string(py, &ydl_info, "uploader")?,
            length: get_u32(py, &ydl_info, "duration")?,
            thumbdefault: get_string(py, &ydl_info, "thumbnail")?,
            likes: get_u32(py, &ydl_info, "like_count")?,
            dislikes: get_u32(py, &ydl_info, "dislike_count")?,
            commentcount: 0, // TODO
            description: get_string(py, &ydl_info, "description")?,
            thumbmedium: get_string(py, &ydl_info, "thumbnail")?,
            thumbhigh: get_string(py, &ydl_info, "thumbnail")?,
            thumbstandard: get_string(py, &ydl_info, "thumbnail")?,
            thumbmaxres: get_string(py, &ydl_info, "thumbnail")?,
            published: get_string(py, &ydl_info, "upload_date")?,
            category: 0, // TODO "categories"?
            streams: allstreams,
            videostreams: videostreams,
            audiostreams: audiostreams,
            _marker: PhantomData,
        })
    }

}

fn get_string(py: Python, dict: &PyDict, key: &str) -> Result<String> {
    Ok(dict.get_item(py, key).ok_or_else(|| format!("{} not found in dict", key))?
       .extract::<String>(py).map_err(|_| format!("{} found in dict but not String", key))?)
}
fn get_u32(py: Python, dict: &PyDict, key: &str) -> Result<u32> {
    Ok(dict.get_item(py, key).unwrap()
       .extract::<u32>(py)?)
}


fn get_streams_with_youtube_dl(py: Python, info: &PyDict) -> Result<(Vec<Stream>, Vec<Stream>, Vec<Stream>)> {
    let formats: PyList = info.get_item(py, "formats").unwrap().extract::<PyList>(py)?;
    let all_stream_infos: Vec<PyDict> = formats.iter(py)
        .map(|obj| obj.extract::<PyDict>(py).unwrap()) // TODO make it return Result<_> which can be `?`d
        .collect();

    let mut all_streams = Vec::new();
    let mut audio_streams = Vec::new();
    let mut video_streams = Vec::new();
    for stream_info in all_stream_infos {
        let stream = Stream::from_py_dict(py, &stream_info)?;
        let vcodec = get_string(py, &stream_info, "vcodec")?;
        let acodec = get_string(py, &stream_info, "acodec")?;
        all_streams.push(stream.clone());
        if acodec != "none" && vcodec == "none" {
            // Audio
            audio_streams.push(stream);
        } else if acodec == "none" && vcodec != "none" {
            // Video
            video_streams.push(stream);
        } else {
            // Normal (?)
        }
    }
    Ok((all_streams, audio_streams, video_streams))
}

fn get_streams(basic: &HashMap<String, String>) -> (Vec<Stream>, Vec<Stream>, Vec<Stream>) {
    let mut parsed_streams: Vec<Stream> = Vec::new();
    let streams: Vec<&str> = basic["url_encoded_fmt_stream_map"]
        .split(',')
        .collect();

    for url in streams.iter() {
        let parsed = ::parse_url(url);
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
            let parsed = ::parse_url(url);
            let extension = &parsed["type"]
                .split('/')
                .nth(1)
                .unwrap()
                .split(';')
                .next()
                .unwrap();
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
                let audio_extension = if extension == &"mp4" {"m4a"} else {extension};
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
