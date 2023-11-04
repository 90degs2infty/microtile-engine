use super::raster::Rasterization;
use array_init::array_init;
use core::result::Result;
use either::Either;

// As of now, computations on const generics are not possible in a somewhat stable manner
// There is [`feature(generic_const_exprs)`](https://github.com/rust-lang/rust/issues/76560), but
// that is _very_ experimental still.
// So instead of having everything nice and generic, I'll hardcode the dimensions of a board.
pub const BOARD_ROWS: usize = 5;
pub const BOARD_COLS: usize = 5;

mod sealed {
    pub trait Seal {}
}

pub trait State: sealed::Seal {}

pub struct TakesTile;

impl sealed::Seal for TakesTile {}
impl State for TakesTile {}

pub struct ProcessesRows;
impl sealed::Seal for ProcessesRows {}
impl State for ProcessesRows {}

pub enum BoardError {
    InvalidPosition,
}

pub struct Board<S> {
    _state: S,
    _grid: [[bool; BOARD_COLS + 2]; BOARD_ROWS + 2], // row major encoding
}

impl Board<TakesTile> {
    #[must_use]
    pub fn new() -> Self {
        let rim = array_init(|r| {
            if r == 0 || r == BOARD_ROWS + 1 {
                [true; BOARD_COLS + 2]
            } else {
                array_init(|c| c == 0 || c == BOARD_COLS + 1)
            }
        });
        Self {
            _state: TakesTile {},
            _grid: rim,
        }
    }

    #[must_use]
    pub fn is_position_valid<T>(&self, _tile: &T) -> bool
    where
        T: Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }>,
    {
        todo!()
    }

    pub fn freeze_tile<T>(self, _tile: &T) -> Result<Board<TakesTile>, BoardError>
    where
        T: Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }>,
    {
        todo!()
    }

    #[must_use]
    pub fn freeze_tile_assume_valid<T>(self, _tile: &T) -> Board<TakesTile>
    where
        T: Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }>,
    {
        todo!()
    }
}

impl Default for Board<TakesTile> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl Board<ProcessesRows> {
    #[must_use]
    pub fn process_row(self) -> Either<Board<ProcessesRows>, Board<TakesTile>> {
        todo!()
    }
}

impl<S> Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }> for Board<S> {
    fn rasterize(&self) -> [[bool; BOARD_ROWS + 2]; BOARD_COLS + 2] {
        todo!()
    }
}
