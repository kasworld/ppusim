use crate::rgba::RGBA;

pub struct RenderDst {
    pub w: usize,
    pub h: usize,
    pub buffer: Vec<RGBA>,
}

impl RenderDst {
    pub fn new_empty(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            buffer: vec![0; w * h],
        }
    }
}
