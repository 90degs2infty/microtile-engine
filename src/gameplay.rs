use crate::geometry::{
    board::{Board, ProcessesRows as BoardProcesses, TakesTile},
    tile::{BasicTile, DisplacedTile, RotatedTile},
};

use either::Either;

mod sealed {
    pub trait Seal {}
}

pub trait State: sealed::Seal {}

pub struct TileNeeded {
    board: Board<TakesTile>,
}

impl sealed::Seal for TileNeeded {}
impl State for TileNeeded {}

impl TileNeeded {
    #[must_use]
    pub fn new() -> Self {
        Self {
            board: Board::default(),
        }
    }
}

impl Default for TileNeeded {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TileFloating {
    tile: DisplacedTile<RotatedTile<BasicTile>>,
    board: Board<TakesTile>,
}

impl TileFloating {
    fn new(tile: BasicTile, board: Board<TakesTile>) -> Self {
        Self {
            tile: DisplacedTile::new(RotatedTile::new(tile)),
            board,
        }
    }
}

impl sealed::Seal for TileFloating {}
impl State for TileFloating {}

pub struct ProcessRows {
    _board: Board<BoardProcesses>,
}

impl sealed::Seal for ProcessRows {}
impl State for ProcessRows {}

pub struct Over {
    _board: Board<TakesTile>,
}

impl sealed::Seal for Over {}
impl State for Over {}

pub struct Game<S, const M: usize, const N: usize> {
    _s: S,
}

impl<const M: usize, const N: usize> Game<TileNeeded, M, N> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            _s: TileNeeded::default(),
        }
    }

    #[must_use]
    pub fn place_tile(
        self,
        _tile: BasicTile,
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
