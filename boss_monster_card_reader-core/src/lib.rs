use image::{imageops, GrayImage};
use image::{GenericImageView, SubImage};

use imageproc::contours::find_contours;
use imageproc::contrast::threshold;
use imageproc::distance_transform::Norm;
use imageproc::geometry::min_area_rect;
use imageproc::morphology::open_mut;
use imageproc::rect::Rect;

pub mod diag;
pub mod helpers;

pub type Image = image::GrayImage;

pub struct CardInfos {
    pub name: String,
    pub description: String,
}

pub fn read_batch(img: &Image, diag: Option<&impl diag::Diagnostic>) -> Vec<CardInfos> {
    //! Extract card information from a scan of cards.
    //!
    //! 1. Extract card rois by executing `find_chunks`
    //! 2. Extract card information using `read_card` for each sub-image (`img(roi)`).

    let card_rois = find_chunks(img, diag);

    if cfg!(feature = "diag_card_finder") {
        if let Some(diag) = diag {
            diag.diag_card_finder(&card_rois);
        }
    }

    let cards_infos: Vec<CardInfos> = card_rois
        .iter()
        .map(|roi| {
            img.view(
                roi.left().try_into().unwrap(),
                roi.top().try_into().unwrap(),
                roi.width(),
                roi.height(),
            )
        })
        .map(|view| read_card(view, diag))
        .collect();

    cards_infos
}

fn find_chunks(img: &Image, diag: Option<&impl diag::Diagnostic>) -> Vec<Rect> {
    //! Extract the roi of each card by looking for contours after a binary threshold.
    //!
    //! 1. Binary threshold to isolate card from white background
    //! 2. Find **external** contours
    //! 3. Retrieve bounding rect of each contours

    // Binary threshold the image
    const THRESH_VAL: u8 = 200;
    let mut bin = threshold(img, THRESH_VAL);
    imageops::invert(&mut bin);

    // Open to remove noise
    const KERN_SIZE: u8 = 15;
    open_mut(&mut bin, Norm::LInf, KERN_SIZE);

    if cfg!(feature = "diag_card_finder") {
        if let Some(diag) = diag {
            diag.diag_card_finder_thresh(&bin);
        }
    }

    // Find external contours
    // Approx contours to rectangles
    let rois: Vec<Rect> = find_contours(&bin)
        .into_iter()
        // .filter(|contour| contour.border_type == BorderType::Outer)
        .filter(|contour| contour.parent.is_none())
        .map(|contour| min_area_rect(&contour.points))
        .map(|rect| {
            let x = rect.iter().map(|p| p.x).min().unwrap();
            let y = rect.iter().map(|p| p.y).min().unwrap();
            let x_max = rect.iter().map(|p| p.x).max().unwrap();
            let y_max = rect.iter().map(|p| p.y).max().unwrap();
            let width = x_max - x;
            let height = y_max - y;
            Rect::at(x, y).of_size(width.try_into().unwrap(), height.try_into().unwrap())
        })
        .collect();

    rois
}

fn read_card(img: SubImage<&GrayImage>, diag: Option<&impl diag::Diagnostic>) -> CardInfos {
    //! Read the information from a card image.

    todo!();
    let card_infos;

    card_infos
}
