// TODO: to be moved to gameplay module

use crate::{
    geometry::grid::{ExtGrid, Grid},
    rendering::{Active, Passive, Rendering},
};
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

#[derive(Debug)]
pub enum BoardError {
    InvalidPosition,
}

pub struct Board<S> {
    state: S,
    grid: ExtGrid,
}

impl Board<TakesTile> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: TakesTile {},
            grid: ExtGrid::RIM,
        }
    }

    #[must_use]
    pub fn is_position_valid<T>(&self, tile: &T) -> bool
    where
        for<'a> ExtGrid: From<&'a T>,
    {
        let raster = ExtGrid::from(tile);

        self.grid.overlaps(&raster)
    }

    /// # Errors
    ///
    /// Returns [`BoardError::InvalidPosition`] iff the specified tile overlaps with occupied
    /// cells.
    pub fn freeze_tile<T>(self, tile: T) -> Result<Board<ProcessesRows>, BoardError>
    where
        for<'a> ExtGrid: From<&'a T>,
        ExtGrid: From<T>,
    {
        if !self.is_position_valid(&tile) {
            return Err(BoardError::InvalidPosition);
        }

        Ok(self.freeze_tile_assume_valid(tile))
    }

    /// # Panics
    ///
    /// TBD
    #[must_use]
    pub fn freeze_tile_assume_valid<T>(self, tile: T) -> Board<ProcessesRows>
    where
        ExtGrid: From<T>,
    {
        let raster = ExtGrid::try_from(tile).unwrap_or_default();

        Board {
            state: ProcessesRows::default(),
            grid: self.grid.union(raster),
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
        let fully_populated = self
            .grid
            .contains(&Grid::ROWS[self.state.current - 1].into());

        // Check next row
        if !fully_populated {
            return Either::Left(Board {
                state: ProcessesRows::new(self.state.current + 1),
                grid: self.grid,
            });
        }

        // Move all rows by one and clear the topmost row
        let shifted = self
            .grid
            .center()
            .discard_and_shift(self.state.current - 1)
            .expect("Row has been checked before");

        // We have to recheck the current row since the row that used to be
        // above might be fully populated, too.
        let next_row = self.state.current;

        Either::Left(Board {
            state: ProcessesRows::new(next_row),
            grid: ExtGrid::from(shifted).union(ExtGrid::RIM),
        })
    }
}

impl Rendering<BOARD_ROWS, BOARD_COLS, Passive> for Board<ProcessesRows> {
    fn render_buf(&self, buffer: &mut [[bool; BOARD_COLS]; BOARD_ROWS]) {
        buffer
            .iter_mut()
            .enumerate()
            .map(|(idx, row)| (idx + 1, row)) // index 0 in buffer corresponds to index 1 in grid
            .filter(|(idx, _)| *idx != self.state.current)
            .for_each(|(idx, row)| row.copy_from_slice(&self.grid[idx][1..=5]));

        buffer[self.state.current - 1] = [false; BOARD_COLS]; // again, adapt for the offset in indices
    }
}

impl Rendering<BOARD_ROWS, BOARD_COLS, Active> for Board<ProcessesRows> {
    fn render_buf(&self, buffer: &mut [[bool; BOARD_COLS]; BOARD_ROWS]) {
        *buffer = [[false; BOARD_COLS]; BOARD_ROWS];

        buffer[self.state.current - 1].copy_from_slice(&self.grid[self.state.current][1..=5]);
    }
}

/*
impl<S> Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }> for Board<S> {
    fn rasterize(&self) -> [[bool; BOARD_ROWS + 2]; BOARD_COLS + 2] {
        self.grid
    }
} */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_rows() {
        let initial_grid = [
            [true; BOARD_COLS + 2],
            [true, false, true, false, true, false, true],
            [true; BOARD_COLS + 2],
            [true, true, false, true, false, true, true],
            [true; BOARD_COLS + 2],
            [true, false, false, false, false, false, true],
            [true; BOARD_COLS + 2],
        ];

        let final_grid = [
            [true; BOARD_COLS + 2],
            [true, false, true, false, true, false, true],
            [true, true, false, true, false, true, true],
            [true, false, false, false, false, false, true],
            [true, false, false, false, false, false, true],
            [true, false, false, false, false, false, true],
            [true; BOARD_COLS + 2],
        ];

        let mut board = Board::<ProcessesRows> {
            state: ProcessesRows::default(),
            grid: initial_grid.into(),
        };

        // Processing all rows takes BOARD_ROWS + 2 iterations (two rows are fully) populated.
        // The last call to `process` will produce an Either::right value
        for iter in 1..=(BOARD_ROWS + 2) {
            board = match board.process_row() {
                Either::Left(board) => board,
                _ => panic!("Board failed to continue processing after iteration {iter}"),
            };
        }

        let board = match board.process_row() {
            Either::Right(board) => board,
            _ => panic!("Board did not detect end of processing"),
        };

        assert_eq!(board.grid, final_grid.into())
    }
}
