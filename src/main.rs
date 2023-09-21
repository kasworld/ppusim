use std::time::Instant;

use ppusim::{palette, render_dst::RenderDst, tile_vec, tilemap_buffer, tilemap_vec};

const DSTW :usize = 1920;
const DSTH :usize = 1080;


fn main() {
    let begin = Instant::now();

    let palette = palette::Palette::new_random();
    let tile_def = tile_vec::TileVec::new_random();
    let tile_map_def = tilemap_vec::TileMapVec::new_random(DSTW,DSTH);
    let tile_map_buffer = tilemap_buffer::TileMapBuffer::new_random();
    let mut rnd_dst = &mut RenderDst::new_empty(DSTW, DSTH);

        
    println!("init {} sec", begin.elapsed().as_secs_f64() );
    let begin = Instant::now();

    rnd_dst = tile_map_def.render2(rnd_dst, &tile_map_buffer, &tile_def, &palette);

    _ = rnd_dst;
    
    println!("render {} sec", begin.elapsed().as_secs_f64());
    let total_render = tile_map_def.calc_render_count(rnd_dst.w, rnd_dst.h);
    println!("total render {}", total_render);

    //print!("{:?}", rnd_dst,);
    // print!(
    //     "{:?} {:?} {:?} {:?} {:?}",
    //     rnd_dst, tile_map_def, tile_map_buffer, tile_def, palette
    // );
}
