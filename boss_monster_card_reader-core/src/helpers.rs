use image::io::Reader as ImageReader;
use std::path::Path;
use thiserror::Error;

use crate::Image;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Error on opening file")]
    IOError(std::io::Error),
    #[error("Decoding error")]
    DecodeError(image::error::ImageError),
}

pub fn load_image(path: &Path) -> Result<Image, LoadError> {
    //! Load an image from the disk
    //!
    //! # Error
    //! - `LoadError::IOError` if the path is unusable
    //! - `LoadError::DecodeError` if the image can't be read

    let img = ImageReader::open(path)
        .map_err(LoadError::IOError)?
        .decode()
        .map_err(LoadError::DecodeError)?
        .into_luma8();

    Ok(img)
}
