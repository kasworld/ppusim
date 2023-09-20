use ppusim::{palette, render_dst::RenderDst, tile_vec, tilemap_buffer, tilemap_vec};

fn main() {
    let palette = palette::Palette::new_empty();
    let tile_def = tile_vec::TileVec::new();
    let tile_map_def = tilemap_vec::TileMapVec::new();
    let tile_map_buffer = tilemap_buffer::TileMapBuffer::new();
    let mut rnd_dst = &mut RenderDst::new_empty(1920, 1080);
    rnd_dst = tile_map_def.render(rnd_dst, &tile_map_buffer, &tile_def, &palette);
    print!(
        "{:?} {:?} {:?} {:?} {:?}",
        rnd_dst, tile_map_def, tile_map_buffer, tile_def, palette
    );
}
