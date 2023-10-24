use crate::geometry::{
    board::Board,
    tile::{BasicTile, DisplacedTile, RotatedTile},
};

use either::Either;

pub struct TileNeeded {}
pub struct TileFloating {}
pub struct Over {}

// TODO trait sealing

pub trait State {
    type Tile;
}

impl State for TileNeeded {
    type Tile = ();
}

impl State for TileFloating {
    type Tile = DisplacedTile<RotatedTile<BasicTile>>;
}

impl State for Over {
    type Tile = ();
}

pub struct Game<T, const M: usize, const N: usize>
where
    T: State,
{
    tile: T::Tile,
    board: Board<M, N>,
}

impl<const M: usize, const N: usize> Game<TileNeeded, M, N> {
    pub fn new() -> Self {
        Self {
            tile: (),
            board: [[false; N]; M],
        }
    }

    pub fn place_tile(
        tile: <TileFloating as State>::Tile,
    ) -> Either<Game<TileFloating, M, N>, Game<Over, M, N>> {
        todo!("To be implemented")
    }
}
