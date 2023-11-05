use crate::geometry::{
    board::{Board, ProcessesRows as BoardProcesses, TakesTile, BOARD_COLS, BOARD_ROWS},
    tile::{BasicTile, Dimensionee, DisplacedTile, Displacee, RotatedTile},
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
    fn new(board: Board<TakesTile>) -> Self {
        Self { board }
    }
}

impl Default for TileNeeded {
    fn default() -> Self {
        Self::new(Board::default())
    }
}

pub struct TileFloating {
    tile: DisplacedTile<RotatedTile<BasicTile>>,
    board: Board<TakesTile>,
}

impl TileFloating {
    fn new(tile: DisplacedTile<RotatedTile<BasicTile>>, board: Board<TakesTile>) -> Self {
        Self { tile, board }
    }
}

impl sealed::Seal for TileFloating {}
impl State for TileFloating {}

pub struct ProcessRows {
    board: Board<BoardProcesses>,
}

impl ProcessRows {
    fn new(board: Board<BoardProcesses>) -> Self {
        Self { board }
    }
}

impl sealed::Seal for ProcessRows {}
impl State for ProcessRows {}

pub struct Over {
    board: Board<TakesTile>,
}

impl Over {
    fn new(board: Board<TakesTile>) -> Self {
        Self { board }
    }
}

impl sealed::Seal for Over {}
impl State for Over {}

pub struct Game<S> {
    s: S,
}

impl Game<TileNeeded> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            s: TileNeeded::default(),
        }
    }

    #[must_use]
    pub fn place_tile(self, tile: BasicTile) -> Either<Game<TileFloating>, Game<Over>> {
        let (_, height) = tile.dimensions();
        let tile = DisplacedTile::new(RotatedTile::new(tile)).displace_by(
            (BOARD_COLS / 2).try_into().unwrap(),
            (BOARD_ROWS - height).try_into().unwrap(),
        );
        if self.s.board.is_position_valid(&tile) {
            Either::Left(Game {
                s: TileFloating::new(tile, self.s.board),
            })
        } else {
            Either::Right(Game {
                s: Over::new(self.s.board),
            })
        }
    }
}

impl Default for Game<TileNeeded> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl Game<TileFloating> {
    #[must_use]
    pub fn descend_tile(self) -> Either<Game<TileFloating>, Game<ProcessRows>> {
        let candidate = self.s.tile.clone().displace_by(0, -1);

        if self.s.board.is_position_valid(&candidate) {
            Either::Left(Game {
                s: TileFloating::new(candidate, self.s.board),
            })
        } else {
            let board = self.s.board.freeze_tile(self.s.tile).unwrap();
            Either::Right(Game {
                s: ProcessRows::new(board),
            })
        }
    }

    /// Tries to move the tile horizontally to `column`.
    ///
    /// If moving the tile to `column` is not valid, the tile is moved as far as possible.
    ///
    /// # Panics
    ///
    /// If specified column cannot be converted to an `i32`, i.e. if
    /// `let _ : i32 = column.try_into().unwrap()` panics.
    pub fn move_tile_up_to(&mut self, column: u32) {
        let column: i32 = column.try_into().unwrap();
        let mut direction = (column - self.s.tile.displ_x()).signum();
        let mut candidate = self.s.tile.clone().displace_by(direction, 0);

        while direction != 0 && self.s.board.is_position_valid(&candidate) {
            self.s.tile = candidate;
            direction = (column - self.s.tile.displ_x()).signum();
            candidate = self.s.tile.clone().displace_by(direction, 0);
        }
    }
}

impl Game<ProcessRows> {
    #[must_use]
    pub fn process_row(self) -> Either<Game<ProcessRows>, Game<TileNeeded>> {
        match self.s.board.process_row() {
            Either::Left(board) => Either::Left(Game {
                s: ProcessRows::new(board),
            }),
            Either::Right(board) => Either::Right(Game {
                s: TileNeeded::new(board),
            }),
        }
    }
}
