use std::{mem, slice};

pub type RGBA = u32;

pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
    u32::from_ne_bytes([r, g, b, a])
}

pub fn as_u8_slice(v: &[RGBA]) -> &[u8] {
    let element_size = mem::size_of::<RGBA>();
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * element_size) }
}

pub fn from_u8(v: Vec<u8>) -> Vec<RGBA> {
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
