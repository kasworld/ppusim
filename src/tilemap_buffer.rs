use std::{
    fs::File,
    io::{Read, Write},
    mem, slice,
};

// hole lower tile vec index
pub type TileVecIndex = u8;

pub const TILE_MAP_BUFFER_SIZE: usize = 65536 * 11;

pub type TileMapBuffer = Vec<TileVecIndex>;

pub fn new_empty() -> TileMapBuffer {
    vec![0; TILE_MAP_BUFFER_SIZE]
}

pub fn save(tmb :&TileMapBuffer) {
    let mut f = File::create(get_filename()).unwrap();
    f.write_all(as_u8_slice(tmb)).unwrap();
}

pub fn load( ) ->TileMapBuffer {
    let mut f = match File::open(get_filename()) {
        Ok(f) => f,
        Err(err) => {
            println!("skip load file {}, {err}", get_filename());
            return new_empty();
        }
    };
    let mut bytes = Vec::new();

    f.read_to_end(&mut bytes).unwrap();

    from_u8(bytes)
}

fn get_filename() -> String {
    "tilemapbuffer.data".to_owned()
}

fn as_u8_slice(v: &[TileVecIndex]) -> &[u8] {
    let element_size = mem::size_of::<TileVecIndex>();
    unsafe { slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * element_size) }
}

fn from_u8(v: Vec<u8>) -> Vec<TileVecIndex> {
    let data = v.as_ptr();
    let len = v.len();
    let capacity = v.capacity();
    let element_size = mem::size_of::<TileVecIndex>();

    // Make sure we have a proper amount of capacity (may be overkill)
    assert_eq!(capacity % element_size, 0);
    // Make sure we are going to read a full chunk of stuff
    assert_eq!(len % element_size, 0);

    unsafe {
        // Don't allow the current vector to be dropped
        // (which would invalidate the memory)
        mem::forget(v);

        Vec::from_raw_parts(
            data as *mut TileVecIndex,
            len / element_size,
            capacity / element_size,
        )
    }
}
