use std::path::PathBuf;

use image::buffer::ConvertBuffer;

use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;

use boss_monster_card_reader_core::diag::Diagnostic;
use boss_monster_card_reader_core::Image;

pub struct CliDiag<'a> {
    pub output_path: PathBuf,
    pub input_image: &'a Image,
}

impl<'a> Diagnostic for CliDiag<'a> {
    fn diag_card_finder_thresh(&self, bin: &Image) {
        const FILE_NAME: &str = "diag_bin.png";

        // Check folder
        std::fs::create_dir_all(self.output_path.as_path()).unwrap();

        let file_path = self.output_path.join(FILE_NAME);
        bin.save(&file_path).unwrap();
    }

    fn diag_card_finder(&self, rois: &Vec<Rect>) {
        const FILE_NAME: &str = "diag_card_region.png";
        const ROI_COLOR: [u8; 3] = [255, 0, 0];

        // Check folder
        std::fs::create_dir_all(self.output_path.as_path()).unwrap();

        let file_path = self.output_path.join(FILE_NAME);

        let mut img: image::RgbImage = self.input_image.convert();
        for roi in rois {
            draw_hollow_rect_mut(&mut img, roi.clone(), ROI_COLOR.into());
        }

        img.save(&file_path).unwrap();
    }
}
