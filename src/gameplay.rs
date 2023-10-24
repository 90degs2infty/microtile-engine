use crate::geometry::{
    board::Board,
    tile::{BasicTile, DisplacedTile, RotatedTile},
};

struct Game<const M: usize, const N: usize> {
    tile: Option<DisplacedTile<RotatedTile<BasicTile>>>,
    board: Board<M, N>,
}
