use opencv::highgui as gui;
use opencv::imgproc::{bounding_rect, find_contours_def, morphology_ex_def, threshold};
use opencv::prelude::*;
use thiserror::Error;

pub mod helpers;

pub type Image = opencv::core::Mat_<u8>;
pub type Roi = opencv::core::Rect_<i32>;
type Curve = opencv::core::Vector<opencv::core::Vector<opencv::core::Point>>;

pub struct CardInfos {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Error)]
pub enum ReadBatchError {
    #[error("Error while finding card chunks")]
    FindChunk(opencv::Error),
    #[error("Error while reading card information")]
    ReadCard(opencv::Error),
}

pub fn read_batch(img: &Image) -> Result<Vec<CardInfos>, ReadBatchError> {
    //! Extract card information from a scan of cards.
    //!
    //! 1. Extract card rois by executing `find_chunks`
    //! 2. Extract card information using `read_card` for each sub-image (`img(roi)`).

    let card_rois = find_chunks(img).map_err(|e| ReadBatchError::FindChunk(e))?;

    if cfg!(all(debug_assertions, feature = "debug_card_finder")) {
        const WINDOW_NAME: &str = "Debug card finder";
        gui::named_window(WINDOW_NAME, gui::WINDOW_NORMAL).unwrap();
        for roi in &card_rois {
            let card_img = Mat::roi(img.as_untyped(), roi.clone()).unwrap();
            gui::imshow(WINDOW_NAME, &card_img).unwrap();
            gui::wait_key(0).unwrap();
        }
    }

    let cards_infos = card_rois
        .iter()
        .map(|roi| Mat::roi(img.as_untyped(), roi.clone()).unwrap())
        .map(|card| card.try_into_typed::<u8>().unwrap())
        .map(|card| read_card(&card))
        .collect::<Result<Vec<CardInfos>, _>>()
        .map_err(|e| ReadBatchError::ReadCard(e));

    cards_infos
}

fn find_chunks(img: &Image) -> Result<Vec<Roi>, opencv::Error> {
    //! Extract the roi of each card by looking for contours after a binary threshold.
    //!
    //! 1. Binary threshold to isolate card from white background
    //! 2. Find **external** contours
    //! 3. Retrieve bounding rect of each contours

    // Binary threshold the image
    let mut bin = {
        const THRESH_VAL: u8 = 200;

        let mut res = Mat::default().try_into_typed::<u8>().unwrap();
        threshold(
            img,
            &mut res,
            THRESH_VAL.into(),
            u8::MAX.into(),
            opencv::imgproc::THRESH_BINARY_INV,
        )?;
        res
    };

    // Open to remove noise
    bin = {
        const KERN_SIZE: (i32, i32) = (15, 15);
        let kern = Mat::ones(KERN_SIZE.0, KERN_SIZE.1, opencv::core::CV_8U)
            .unwrap()
            .to_mat()
            .unwrap()
            .try_into_typed::<u8>()
            .unwrap();
        let mut res = Mat::default();
        morphology_ex_def(&bin, &mut res, opencv::imgproc::MORPH_OPEN, &kern)?;

        res.try_into_typed::<u8>().unwrap()
    };

    // find contours
    let contours = {
        let mut contours = Curve::default();
        find_contours_def(
            &bin,
            &mut contours,
            opencv::imgproc::RETR_EXTERNAL,
            opencv::imgproc::CHAIN_APPROX_SIMPLE,
        )?;

        contours
    };

    // Approx contours to rectangles
    let rois = contours
        .iter()
        .map(|contour| {
            let roi = bounding_rect(&contour);
            roi
        })
        .collect::<opencv::Result<Vec<Roi>>>()?;

    if cfg!(all(debug_assertions, feature = "debug_card_finder")) {
        const WINDOW_NAME: &str = "Debug card finder";
        const CONTOURS_COLOR: (i32, i32, i32) = (0, 0, 255);
        const ROI_COLOR: (i32, i32, i32) = (255, 0, 0);

        println!("Debug card finder");
        let mut img = {
            let mut res = Mat::default();
            opencv::imgproc::cvt_color_def(img, &mut res, opencv::imgproc::COLOR_GRAY2BGR).unwrap();
            res.try_into_typed::<opencv::core::Vec3b>().unwrap()
        };
        opencv::imgproc::draw_contours_def(&mut img, &contours, -1, CONTOURS_COLOR.into()).unwrap();
        for rect in &rois {
            opencv::imgproc::rectangle_def(&mut img, rect.clone(), ROI_COLOR.into()).unwrap();
        }
        gui::named_window(WINDOW_NAME, gui::WINDOW_NORMAL).unwrap();
        gui::imshow(WINDOW_NAME, &img).unwrap();
        gui::wait_key(0).unwrap();
    }

    Ok(rois)
}

fn read_card(img: &Image) -> Result<CardInfos, opencv::Error> {
    //! Read the information from a card image.

    todo!();
    let card_infos;

    card_infos
}
