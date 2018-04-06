pub use failure::{Fail, Error, err_msg};


#[derive(Fail, Debug)]
pub enum RafyErr {
    #[fail(display = "Video not found (url {})", _0)]
    VideoNotFound (String),
    // #[fail(display = "Network request failed")]
    // NetworkRequestFailed,
}
