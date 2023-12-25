use image::imageops;
use image::GenericImageView;

use imageproc::contours::find_contours;
use imageproc::contrast::{threshold, threshold_mut};
use imageproc::distance_transform::Norm;
use imageproc::geometry::min_area_rect;
use imageproc::morphology::{close_mut, open_mut};
use imageproc::rect::Rect;

pub mod diag;
pub mod helpers;

pub type Image = image::GrayImage;

pub struct CardInfos {
    pub name: String,
    pub description: String,
}

pub struct CardInfosTextChunks {
    pub name: Rect,
    pub description: Rect,
}

pub struct CardInfosSubImages {
    pub name: Image,
    pub description: Image,
}

pub fn read_batch(img: &Image, diag: Option<&impl diag::Diagnostic>) -> Vec<CardInfos> {
    //! Extract card information from a scan of cards.
    //!
    //! 1. Extract card rois by executing `find_chunks`
    //! 2. Extract card information using `read_card` for each sub-image (`img(roi)`).

    let card_rois = find_chunks(img, diag);

    if cfg!(feature = "diag_card_finder") {
        if let Some(diag) = diag {
            diag.diag_card_finder(img, &card_rois);
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
            .to_image()
        })
        .map(|view| {
            let chunks = find_text_chunks(&view, diag);
            (view, chunks)
        })
        .map(|(view, chunks)| extract_infos_subviews(&view, chunks))
        .map(|info_view| read_card(info_view, diag))
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
    close_mut(&mut bin, Norm::LInf, KERN_SIZE);

    if cfg!(feature = "diag_card_finder") {
        if let Some(diag) = diag {
            diag.diag_card_finder_thresh(&bin);
        }
    }

    // Find external contours
    // Approx contours to rectangles
    let rois: Vec<Rect> = find_contours(&bin)
        .into_iter()
        .filter(|contour| contour.border_type == imageproc::contours::BorderType::Outer)
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

fn find_text_chunks(img: &Image, diag: Option<&impl diag::Diagnostic>) -> CardInfosTextChunks {
    //! Extract text chunks based **only** on a fixed position extracted from a reference image.

    let name_roi = {
        // Measures taken on the real card
        const LU_CORNER_OFFSET_RATIO: (f32, f32) =
            (136.0 / 732.0, (116.0 - 54.0) / (1115.0 - 54.0));
        const DIM_RATIO: (f32, f32) = ((657.0 - 136.0) / 732.0, (216.0 - 116.0) / (1115.0 - 54.0));

        let x = img.width() as f32 * LU_CORNER_OFFSET_RATIO.0;
        let y = img.height() as f32 * LU_CORNER_OFFSET_RATIO.1;
        let width = img.width() as f32 * DIM_RATIO.0;
        let height = img.height() as f32 * DIM_RATIO.1;

        Rect::at(x as _, y as _).of_size(width as _, height as _)
    };

    let description_roi = {
        const LU_CORNER_OFFSET_RATIO: (f32, f32) = (62.0 / 732.0, (750.0 - 54.0) / (1115.0 - 54.0));
        const DIM_RATIO: (f32, f32) = ((653.0 - 62.0) / 732.0, (917.0 - 750.0) / (1115.0 - 54.0));

        let x = img.width() as f32 * LU_CORNER_OFFSET_RATIO.0;
        let y = img.height() as f32 * LU_CORNER_OFFSET_RATIO.1;
        let width = img.width() as f32 * DIM_RATIO.0;
        let height = img.height() as f32 * DIM_RATIO.1;

        Rect::at(x as _, y as _).of_size(width as _, height as _)
    };

    let chunks = CardInfosTextChunks {
        name: name_roi,
        description: description_roi,
    };

    if cfg!(feature = "diag_find_text_chunks") {
        if let Some(diag) = diag {
            diag.diag_find_text_chunks(img, &chunks);
        }
    }

    chunks
}

fn extract_infos_subviews(view: &Image, chunks: CardInfosTextChunks) -> CardInfosSubImages {
    let name_sub_img = {
        let rect = chunks.name;
        let view = view.view(
            rect.left() as _,
            rect.top() as _,
            rect.width(),
            rect.height(),
        );
        view.to_image()
    };

    let description_sub_img = {
        let rect = chunks.description;
        let view = view.view(
            rect.left() as _,
            rect.top() as _,
            rect.width(),
            rect.height(),
        );
        view.to_image()
    };

    CardInfosSubImages {
        name: name_sub_img,
        description: description_sub_img,
    }
}

fn read_card(views: CardInfosSubImages, _diag: Option<&impl diag::Diagnostic>) -> CardInfos {
    //! Read the information from a card image.

    use leptess::LepTess;

    const THRESH_VAL: u8 = 200;
    // TODO: Set value
    const TRAINED_DATA_DIR: Option<&str> = None;
    const TRAINED_DATA_NAME: &str = "fra";

    let mut reader = LepTess::new(TRAINED_DATA_DIR, TRAINED_DATA_NAME).unwrap();

    let name = {
        let mut subimg = views.name;
        threshold_mut(&mut subimg, THRESH_VAL);

        let mut raw_data = Vec::new();
        subimg
            .write_to(
                &mut std::io::Cursor::new(&mut raw_data),
                image::ImageOutputFormat::Tiff,
            )
            .unwrap();

        reader.set_image_from_mem(&raw_data).unwrap();
        let mut text = reader.get_utf8_text().unwrap();

        text = text.split_once('\n').unwrap().0.to_string();

        text
    };

    let description = {
        let mut subimg = views.description;
        threshold_mut(&mut subimg, THRESH_VAL);

        let mut raw_data = Vec::new();
        subimg
            .write_to(
                &mut std::io::Cursor::new(&mut raw_data),
                image::ImageOutputFormat::Tiff,
            )
            .unwrap();

        reader.set_image_from_mem(&raw_data).unwrap();
        let mut text = reader.get_utf8_text().unwrap();

        text = text.replace('\n', " ");
        text = text.trim().to_string();

        text
    };

    CardInfos { name, description }
}
