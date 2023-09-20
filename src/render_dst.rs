use crate::rgba::{self, RGBA};

use std::{
    fs::File,
    io::{Read, Write}
};

#[derive(Debug)]
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
    pub fn save(self) {
        let mut f = File::create(get_filename()).unwrap();
        f.write_all(rgba::as_u8_slice(&self.buffer)).unwrap();
    }

    pub fn load(&mut self) {
        let mut f = match File::open(get_filename()) {
            Ok(f) => f,
            Err(err) => {
                println!("skip load file {}, {err}", get_filename());
                return;
            }
        };
        let mut bytes = Vec::new();

        f.read_to_end(&mut bytes).unwrap();

        self.buffer = rgba::from_u8(bytes)
    }
}

fn get_filename() -> String {
    "renderdst.data".to_owned()
}
