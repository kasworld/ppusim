use crate::rgba::RGBA;
use std::{
    fs::File,
    io::{Read, Write},
    mem, slice,
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
        f.write_all(as_u8_slice(&self.buffer)).unwrap();
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

        self.buffer = from_u8(bytes)
    }
}

fn get_filename() -> String {
    "renderdst.data".to_owned()
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
