use microtile_engine::geometry::{
    raster::Rasterization,
    tile::{BasicTile, DisplacedTile, Displacee, RotatedTile, Rotatee},
};

fn print_raster<const M: usize, const N: usize>(raster: &[[bool; N]; M]) {
    println!("x-----x");
    for row in (0..M).rev() {
        print!("|");

        for col in 0..N {
            if raster[row][col] {
                print!("o");
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
    println!("x-----x");
}

fn main() {
    let tile = DisplacedTile::new(RotatedTile::new(BasicTile::Line));
    let raster: [[bool; 5]; 5] = tile.rasterize();
    print_raster(&raster);

    let tile = tile.displace_by(2, 1).rotate_ccw();
    let raster: [[bool; 5]; 5] = tile.rasterize();
    print_raster(&raster);
}
