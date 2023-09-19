use crate::rgba::RGBA;

pub struct RenderDst {
    pub w: usize,
    pub h: usize,
    pub buffer: Vec<RGBA>,
}

impl RenderDst {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            buffer: vec![RGBA::new(0, 0, 0, 0); w * h],
        }
    }
}
