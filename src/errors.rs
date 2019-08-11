use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug, Default)]
pub struct VideoNotFound {}

impl Display for VideoNotFound {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Video not found!").unwrap();
        Ok(())
    }
}

impl Error for VideoNotFound {}

#[derive(Debug, Default)]
pub struct VideoUnavailable {}

impl Display for VideoUnavailable {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Video is unavailable").unwrap();
        Ok(())
    }
}

impl Error for VideoUnavailable {}

