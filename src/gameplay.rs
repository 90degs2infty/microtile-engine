use crate::geometry::{
    board::Board,
    tile::{BasicTile, DisplacedTile, RotatedTile},
};

use either::Either;

pub struct TileNeeded {}

pub struct TileFloating {}

pub struct Over {}

// TODO maybe const generic activeRow: usize
pub struct ProcessRows {}

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

impl State for ProcessRows {
    type Tile = ();
}

pub struct Game<T, const M: usize, const N: usize>
where
    T: State,
{
    _tile: T::Tile,
    _board: Board<M, N>,
}

impl<const M: usize, const N: usize> Game<TileNeeded, M, N> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            _tile: (),
            _board: [[false; N]; M],
        }
    }

    #[must_use]
    pub fn place_tile(
        self,
        _tile: <TileFloating as State>::Tile,
    ) -> Either<Game<TileFloating, M, N>, Game<Over, M, N>> {
        todo!("To be implemented")
    }
}

impl<const M: usize, const N: usize> Default for Game<TileNeeded, M, N> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl<const M: usize, const N: usize> Game<TileFloating, M, N> {
    #[must_use]
    pub fn descend_tile(self) -> Either<Game<TileFloating, M, N>, Game<ProcessRows, M, N>> {
        todo!()
    }

    /// Tries to move the tile horizontally to `column`.
    ///
    /// If moving the tile to `column` is not valid, the tile is moved as far as possible.
    pub fn move_tile_up_to(&mut self, _column: usize) {
        todo!()
    }
}

impl<const M: usize, const N: usize> Game<ProcessRows, M, N> {
    #[must_use]
    pub fn process_row(self) -> Either<Game<ProcessRows, M, N>, Game<TileNeeded, M, N>> {
        todo!()
    }
}
