use imageproc::rect::Rect;

use crate::{CardInfos, Image};

pub trait Diagnostic {
    fn diag_card_finder(&self, _rois: &Vec<Rect>) {}
    fn diag_card_finder_thresh(&self, _bin: &Image) {}
    fn diag_card_reading(&self, _idx: usize, _infos: &CardInfos) {}
}
