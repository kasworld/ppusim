use std::time::Instant;

use ppusim::{palette, render_dst::RenderDst, tile_vec, tilemap_buffer, tilemap_vec};

fn main() {
    let begin = Instant::now();

    let palette = palette::Palette::new_random();
    let tile_def = tile_vec::TileVec::new_random();
    let tile_map_def = tilemap_vec::TileMapVec::new_random();
    let tile_map_buffer = tilemap_buffer::TileMapBuffer::new_random();
    let mut rnd_dst = &mut RenderDst::new_empty(1920, 1080);

        
    println!("init {:?}", Instant::now() - begin);
    let begin = Instant::now();

    rnd_dst = tile_map_def.render(rnd_dst, &tile_map_buffer, &tile_def, &palette);

    _ = rnd_dst;
    
    println!("render {:?}", Instant::now() - begin);

    //print!("{:?}", rnd_dst,);
    // print!(
    //     "{:?} {:?} {:?} {:?} {:?}",
    //     rnd_dst, tile_map_def, tile_map_buffer, tile_def, palette
    // );
}
