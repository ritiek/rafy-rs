use err::*;
use video::*;

use std::str;
use cpython::{Python, PyDict, ObjectProtocol};
use std::marker::PhantomData;


/// The rationale behind `PlaylistElement` is that it takes some time to get the info for each
/// `Video`. Hence this should happen only at the programmer's request.
pub struct PlaylistElement<B: Backend> {
    pub url: String,
    _marker: PhantomData<B>,
}
impl<B: Backend> PlaylistElement<B> {
    pub fn new(url: &str) -> PlaylistElement<B> {
        PlaylistElement {
            url: url.to_string(),
            _marker: PhantomData,
        }
    }
    pub fn video(self) -> Result<Video<B>> {
        Video::<B>::new(&self.url)
    }
}


pub struct Playlist<B: Backend> {
    pub videos: Vec<PlaylistElement<B>>,
    /// List of IDs of videos that were deleted or went private
    pub deleted_videos: Vec<String>,
}

impl<B: Backend> Playlist<B> {
    pub fn new(url: &str) -> Result<Playlist<B>> {
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
                kwargs.set_item(py, "extract_flat", py.True())?; // Makes it not try to download, but just get information
                youtube_dl.get(py, "YoutubeDL")?.call(py, (kwargs,), None)?
            };
            let kwargs = PyDict::new(py);
            kwargs.set_item(py, "download", py.False())?;
            ydl.call_method(py, "extract_info", (url,), Some(&kwargs))?
                .cast_into::<PyDict>(py).unwrap()
        };
        if ydl_info.get_item(py, "_type").unwrap().extract::<String>(py)? != "playlist" {
            bail!("Received dict is not of _type playlist.");
        }
        let entries = ydl_info.get_item(py, "entries").unwrap().extract::<Vec<PyDict>>(py)?;


        let mut videos = Vec::new();
        let mut deleted_videos = Vec::new();
        for entry in &entries {
            {
                println!("== ITEM ==");
                for (key, val) in entry.items(py) {
                    println!("'{}' = {}", key, val);
                }
            }

            let title = entry.get_item(py, "title").unwrap().extract::<String>(py)?;
            let id = entry.get_item(py, "url").unwrap().extract::<String>(py)?;
            if title == "[Private video]" || title == "[Deleted video]" {
                deleted_videos.push(id);
            } else {
                videos.push(
                    PlaylistElement {
                        url: format!("https://www.youtube.com/watch?v={}", id),
                        _marker: PhantomData,
                    });
            }
        }

        Ok(Playlist {
            videos: videos,
            deleted_videos: deleted_videos,
        })
    }
}
