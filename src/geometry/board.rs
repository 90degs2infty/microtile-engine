use crate::{
    geometry::raster::Rasterization,
    rendering::{Active, Passive, Rendering},
};
use array_init::{array_init, from_iter};
use core::{iter::zip, result::Result};
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

pub struct ProcessesRows {
    current: usize,
}

impl ProcessesRows {
    fn new(current: usize) -> Self {
        Self { current }
    }
}

impl sealed::Seal for ProcessesRows {}
impl State for ProcessesRows {}

impl Default for ProcessesRows {
    fn default() -> Self {
        Self::new(1)
    }
}

pub enum BoardError {
    InvalidPosition,
}

pub struct Board<S> {
    state: S,
    grid: [[bool; BOARD_COLS + 2]; BOARD_ROWS + 2], // row major encoding
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
            state: TakesTile {},
            grid: rim,
        }
    }

    #[must_use]
    pub fn is_position_valid<T>(&self, tile: &T) -> bool
    where
        T: Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }>,
    {
        let raster = tile.rasterize();

        zip(raster, &self.grid)
            .map(|(a, b)| zip(a, b).map(|(a, b)| a && *b).all(|b| b))
            .all(|b| b)
    }

    /// # Errors
    ///
    /// Returns [`BoardError::InvalidPosition`] iff the specified tile overlaps with occupied
    /// cells.
    pub fn freeze_tile<T>(self, tile: &T) -> Result<Board<ProcessesRows>, BoardError>
    where
        T: Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }>,
    {
        if !self.is_position_valid(tile) {
            return Err(BoardError::InvalidPosition);
        }

        Ok(self.freeze_tile_assume_valid(tile))
    }

    /// # Panics
    ///
    /// TBD
    #[must_use]
    pub fn freeze_tile_assume_valid<T>(self, tile: &T) -> Board<ProcessesRows>
    where
        T: Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }>,
    {
        let raster = tile.rasterize();

        let rows = zip(raster, self.grid).map(|(a, b)| {
            let row = zip(a, b).map(|(a, b)| a || b);
            from_iter(row).unwrap()
        });

        let grid = from_iter(rows).unwrap();

        Board {
            state: ProcessesRows::default(),
            grid,
        }
    }
}

impl Default for Board<TakesTile> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl Rendering<BOARD_ROWS, BOARD_COLS, Passive> for Board<TakesTile> {
    fn render_buf(&self, buffer: &mut [[bool; BOARD_COLS]; BOARD_ROWS]) {
        buffer
            .iter_mut()
            .enumerate()
            .for_each(|(idx, row)| row.copy_from_slice(&self.grid[idx][1..=5]));
    }
}

impl Board<ProcessesRows> {
    #[must_use]
    pub fn process_row(mut self) -> Either<Board<ProcessesRows>, Board<TakesTile>> {
        if self.state.current > BOARD_ROWS {
            return Either::Right(Board {
                state: TakesTile {},
                grid: self.grid,
            });
        }

        // Check current row for being fully populated
        let fully_populated = self.grid[self.state.current].iter().all(|b| *b);

        // Check next row
        if !fully_populated {
            return Either::Left(Board {
                state: ProcessesRows::new(self.state.current + 1),
                grid: self.grid,
            });
        }

        // Move all rows by one and clear the topmost row
        for r in self.state.current..BOARD_ROWS {
            self.grid.copy_within((r + 1)..(r + 2), r);
        }

        self.grid[BOARD_ROWS] = array_init(|c| c == 0 || c == BOARD_COLS + 1);

        let next_row = self.state.current;

        Either::Left(Board {
            state: ProcessesRows::new(next_row),
            grid: self.grid,
        })
    }
}

impl Rendering<BOARD_ROWS, BOARD_COLS, Passive> for Board<ProcessesRows> {
    fn render_buf(&self, _buffer: &mut [[bool; BOARD_COLS]; BOARD_ROWS]) {
        todo!()
    }
}

impl Rendering<BOARD_ROWS, BOARD_COLS, Active> for Board<ProcessesRows> {
    fn render_buf(&self, _buffer: &mut [[bool; BOARD_COLS]; BOARD_ROWS]) {
        todo!()
    }
}

impl<S> Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }> for Board<S> {
    fn rasterize(&self) -> [[bool; BOARD_ROWS + 2]; BOARD_COLS + 2] {
        todo!()
    }
}
