use err::*;

use pbr::ProgressBar;
use std::str;
use hyper::client::response::Response;
use std::io::Read;
use std::io::prelude::*;
use std::fs::File;
use cpython::{Python, PyDict};
/// After creating a `Stream` struct, you can check its attributes or call methods on it.
///
/// # Examples
///
/// ```
/// extern crate rafy;
/// use rafy::Video;
///
/// fn main() {
///     let content = Video::new("https://www.youtube.com/watch?v=DjMkfARvGE8").unwrap();
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
impl Stream {
    pub fn from_py_dict(py: Python, info: &PyDict) -> Result<Stream> {
        let extension = info.get_item(py, "ext").unwrap()
                            .extract::<String>(py)?;
        let quality = info.get_item(py, "abr")
                          .map_or(Ok(0), |obj| obj.extract::<u32>(py))?;
        let url = info.get_item(py, "url").unwrap()
                      .extract::<String>(py)?;
        Ok(Stream {
            extension: extension,
            quality: format!("{}", quality), // TODO: quality is maybe better u32
            url: url,
        })
    }
}

/// Create a `Vec<Stream>` object by calling `Video::new().streams` .
///
/// # Examples
///
/// ```
/// extern crate rafy;
/// use rafy::Video;
///
/// fn main() {
///     let content = Video::new("https://www.youtube.com/watch?v=DjMkfARvGE8").unwrap();
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
    /// use rafy::Video;
    ///
    /// fn main() {
    ///     let content = Video::new("https://www.youtube.com/watch?v=AnRSXHQ2qyo").unwrap();
    ///     let title = content.title;
    ///     let streams = content.streams;
    ///     let ref stream = streams[0];
    ///     // It is necessary to pass the filename to generate in download()
    ///     stream.download(&title);
    ///
    ///     let audiostreams = content.audiostreams;
    ///     let ref audiostream = audiostreams[0];
    ///     audiostream.download(&title);
    ///
    ///     let videostreams = content.videostreams;
    ///     let ref videostream = videostreams[0];
    ///     videostream.download(&title);
    /// }
    /// ```

    pub fn download(&self, title: &str) -> Result<()> {
        let response = ::send_request(&self.url)?;
        let file_size = ::get_file_size(&response);
        let file_name = format!("{}.{}", title, &self.extension);
        Self::write_file(response, &file_name, file_size)?;
        Ok(())
    }

    fn write_file(mut response: Response, title: &str, file_size: u64) -> Result<()> {
        let mut pb = ProgressBar::new(file_size);
        pb.format("╢▌▌░╟");

        let mut buf = [0; 128 * 1024];
        let mut file = File::create(title)?;
        loop {
            match response.read(&mut buf) {
                Ok(len) => {
                    file.write_all(&buf[..len])?;
                    pb.add(len as u64);
                    if len == 0 {
                        break;
                    }
                    len
                }
                Err(why) => bail!("{}", why),
            };
        }
        Ok(())
    }

}
