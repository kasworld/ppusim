use crate::rgba::{RGBA, self};
use rand::Rng;
use std::{
    fs::File,
    io::{Read, Write},
    mem, slice,
};

pub const UPPER_PALETTE_SIZE: usize = 256;

pub const LOWER_PALETTE_SIZE: usize = 256;

pub const PALETTE_SIZE: usize = UPPER_PALETTE_SIZE * LOWER_PALETTE_SIZE;

#[derive(Debug)]
pub struct Palette(Vec<RGBA>);

impl Palette {
    pub fn new_empty() -> Self {
        Self(vec![0; PALETTE_SIZE])
    }
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let mut rtn = Self::new_empty();
        for i in 0..PALETTE_SIZE {
            rtn.0[i] = rng.gen::<u32>();
        }
        rtn
    }
    pub fn get_lower<'a>(&'a self, hi: u8) -> &'a [RGBA] {
        let start = hi as usize * LOWER_PALETTE_SIZE;
        &self.0[start..start + LOWER_PALETTE_SIZE]
    }

    pub fn save(self) {
        let mut f = File::create(get_filename()).unwrap();
        f.write_all(rgba::as_u8_slice(&self.0)).unwrap();
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

        self.0 = rgba::from_u8(bytes)
    }
}

fn get_filename() -> String {
    "palette.data".to_owned()
}

