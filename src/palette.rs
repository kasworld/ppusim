use crate::rgba::RGBA;
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
        f.write_all(as_u8_slice(&self.0)).unwrap();
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

        self.0 = from_u8(bytes)
    }
}

fn get_filename() -> String {
    "palette.data".to_owned()
}

fn as_u8_slice(v: &[RGBA]) -> &[u8] {
    let element_size = mem::size_of::<RGBA>();
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * element_size) }
}

fn from_u8(v: Vec<u8>) -> Vec<RGBA> {
    let data = v.as_ptr();
    let len = v.len();
    let capacity = v.capacity();
    let element_size = mem::size_of::<RGBA>();

    // Make sure we have a proper amount of capacity (may be overkill)
    assert_eq!(capacity % element_size, 0);
    // Make sure we are going to read a full chunk of stuff
    assert_eq!(len % element_size, 0);

    unsafe {
        // Don't allow the current vector to be dropped
        // (which would invalidate the memory)
        mem::forget(v);

        Vec::from_raw_parts(
            data as *mut RGBA,
            len / element_size,
            capacity / element_size,
        )
    }
}
