use opencv::highgui as gui;
use opencv::imgcodecs::{imread, IMREAD_GRAYSCALE};
use std::path::Path;
use thiserror::Error;

use crate::Image;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("The used path is not utf-8")]
    UnusablePath,
    #[error("Error while reading image")]
    ReadError(opencv::Error),
}

pub fn load_image(path: &Path) -> Result<Image, LoadError> {
    let path = path.to_str().ok_or(LoadError::UnusablePath)?;
    let img = imread(path, IMREAD_GRAYSCALE)
        .map_err(|err| LoadError::ReadError(err))?
        .try_into_typed::<u8>()
        .unwrap();

    if cfg!(all(debug_assertions, feature = "debug_imread")) {
        const WINDOW_NAME: &str = "Debug imread";

        println!("{}", WINDOW_NAME);
        gui::named_window(WINDOW_NAME, gui::WINDOW_NORMAL).unwrap();
        gui::imshow(WINDOW_NAME, &img).unwrap();
        gui::wait_key(0).unwrap();
    }

    Ok(img)
}
