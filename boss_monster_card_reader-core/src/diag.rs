use imageproc::rect::Rect;

use crate::{CardInfosTextChunks, Image};

pub trait Diagnostic {
    fn diag_card_finder(&self, _src_img: &Image, _rois: &Vec<Rect>) {}
    fn diag_card_finder_thresh(&self, _bin: &Image) {}
    fn diag_find_text_chunks(&self, _src_img: &Image, _chunks: &CardInfosTextChunks) {}
}
