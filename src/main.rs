use image;
use std::time::Instant;

use ppusim::{
    palette::Palette, tile_vec::TileVec, tilemap_buffer::TileMapBuffer, tilemap_vec::TileMapVec,
};

const DSTW: usize = 1920;
const DSTH: usize = 1080;

fn main() {
    let palette = Palette::new_rainbow();
    palette.save_to_file("palette.bmp".to_owned());
    let tile_def = TileVec::load_from_file("tilesdef.bmp".to_string());
    let tile_map_def = TileMapVec::new_random(DSTW, DSTH);
    let tile_map_buffer = TileMapBuffer::new_seq();

    loop {
        make_frame(&palette, &tile_def, &tile_map_def, &tile_map_buffer);
    }
}

fn make_frame(
    palette: &Palette,
    tile_def: &TileVec,
    tile_map_def: &TileMapVec,
    tile_map_buffer: &TileMapBuffer,
) {
    let mut rnd_dst = &mut image::RgbaImage::new(DSTW as u32, DSTH as u32);
    rnd_dst = tile_map_def.render(rnd_dst, &tile_map_buffer, &tile_def, &palette);
    rnd_dst.save("ppu.bmp").unwrap();
}
