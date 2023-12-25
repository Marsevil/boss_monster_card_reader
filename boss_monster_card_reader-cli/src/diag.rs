use std::cell::Cell;
use std::path::PathBuf;

use image::buffer::ConvertBuffer;

use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

use boss_monster_card_reader_core::diag::Diagnostic;
use boss_monster_card_reader_core::{CardInfosTextChunks, Image};

pub struct CliDiag {
    pub output_path: PathBuf,

    diag_find_text_chunks_thresh_count: Cell<usize>,
}

impl CliDiag {
    pub fn new(output_path: PathBuf) -> Self {
        Self {
            output_path,
            diag_find_text_chunks_thresh_count: Cell::new(0),
        }
    }

    pub fn diag_reading(&self, img: &Image) {
        const FILE_NAME: &str = "read.png";
        let file_path = self.output_path.join(FILE_NAME);

        img.save(file_path).unwrap();
    }
}

impl Diagnostic for CliDiag {
    fn diag_card_finder_thresh(&self, bin: &Image) {
        const FILE_NAME: &str = "diag_bin.png";

        // Check folder
        std::fs::create_dir_all(self.output_path.as_path()).unwrap();

        let file_path = self.output_path.join(FILE_NAME);
        bin.save(&file_path).unwrap();
    }

    fn diag_card_finder(&self, src_img: &Image, rois: &[Rect]) {
        const FILE_NAME: &str = "diag_card_region.png";
        const ROI_COLOR: [u8; 3] = [255, 0, 0];

        // Check folder
        std::fs::create_dir_all(self.output_path.as_path()).unwrap();

        let file_path = self.output_path.join(FILE_NAME);

        let mut img: image::RgbImage = src_img.convert();
        for roi in rois {
            draw_hollow_rect_mut(&mut img, *roi, ROI_COLOR.into());
        }

        img.save(&file_path).unwrap();
    }

    fn diag_find_text_chunks(&self, src_img: &Image, chunks: &CardInfosTextChunks) {
        const ROI_COLOR: [u8; 3] = [255, 0, 0];

        // To avoid conflict file name we use an internal counter for each function call
        let count = self.diag_find_text_chunks_thresh_count.get();
        self.diag_find_text_chunks_thresh_count.set(count + 1);

        let file_name = format!("text_thresh_{}.png", count);

        // Check folder
        std::fs::create_dir_all(self.output_path.as_path()).unwrap();

        let mut img: image::RgbImage = src_img.convert();
        draw_hollow_rect_mut(&mut img, chunks.name, ROI_COLOR.into());
        draw_hollow_rect_mut(&mut img, chunks.description, ROI_COLOR.into());

        let file_path = self.output_path.join(file_name);

        img.save(file_path).unwrap();
    }
}
