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