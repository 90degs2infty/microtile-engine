// TODO: to be moved to gameplay module

use crate::{
    gameplay::raster::{Active, Passive, Rasterization},
    geometry::grid::{ExtGrid, Grid},
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

#[derive(Debug)]
pub struct TakesTile;

impl sealed::Seal for TakesTile {}
impl State for TakesTile {}

#[derive(Debug)]
pub struct ProcessesRows {
    /// 0-indexed, but with respect to a `Grid`'s (as opposed to `ExtGrid`'s) row count.
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
        Self::new(0)
    }
}

#[derive(Debug)]
pub enum BoardError {
    InvalidPosition,
}

#[derive(Debug)]
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
        for<'a> &'a T: TryInto<ExtGrid>,
    {
        if let Ok(raster) = tile.try_into() {
            !self.grid.overlaps(&raster)
        } else {
            false
        }
    }

    /// # Errors
    ///
    /// Returns [`BoardError::InvalidPosition`] iff the specified tile overlaps with occupied
    /// cells.
    pub fn freeze_tile<T>(self, tile: T) -> Result<Board<ProcessesRows>, BoardError>
    where
        for<'a> &'a T: TryInto<ExtGrid>,
        T: TryInto<ExtGrid>,
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
        T: TryInto<ExtGrid>,
    {
        let raster = tile.try_into().unwrap_or_default();

        Board {
            state: ProcessesRows::default(),
            grid: self.grid.union(&raster),
        }
    }
}

impl Default for Board<TakesTile> {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl Rasterization<Passive> for Board<TakesTile> {
    fn rasterize_buf(&self, out: &mut Grid) {
        *out = self.grid.clone().center();
    }
}

impl Board<ProcessesRows> {
    /// Note that if there are 5 rows to check (i.e. no row is fully populated),
    /// only 4 calls to `process_row` are necessary, as `process_row` enumerates
    /// the transitions between rows to check (as opposed to the rows themselves).
    #[must_use]
    pub fn process_row(self) -> Either<Board<ProcessesRows>, Board<TakesTile>> {
        // Note that by design, the bottom row cannot be empty when entering
        // this function (we've just dropped a tile).
        // Consequently, when entering this function we may always first check
        // the current row for being fully populated (as opposed to checking it
        // for being empty).

        // Check current row for being fully populated
        let fully_populated = self
            .grid
            .contains(&Grid::ROWS[self.state.current].clone().into());

        let next_row;
        let pruned_grid;

        // Check next row
        if fully_populated {
            // Move all rows by one and clear the topmost row
            let shifted = self
                .grid
                .center()
                .discard_and_shift(self.state.current)
                .expect("Row has been checked before");

            // We have to recheck the current row since the row that used to be
            // above might be fully populated, too.
            next_row = self.state.current;
            pruned_grid = ExtGrid::from(shifted).union(&ExtGrid::RIM);
        } else {
            next_row = self.state.current + 1;
            pruned_grid = self.grid;
        }

        // Guard lookahead, so that we do not check beyond the board's extend
        if next_row >= BOARD_ROWS {
            return Either::Right(Board {
                state: TakesTile {},
                grid: pruned_grid,
            });
        }

        // There are no empty interleaving rows, so once we encounter an empty row, we can skip
        // ahead
        let next_row_empty = !pruned_grid.overlaps(&Grid::ROWS[next_row].clone().into());
        if next_row_empty {
            Either::Right(Board {
                state: TakesTile {},
                grid: pruned_grid,
            })
        } else {
            Either::Left(Board {
                state: ProcessesRows::new(next_row),
                grid: pruned_grid,
            })
        }
    }
}

impl Rasterization<Passive> for Board<ProcessesRows> {
    fn rasterize_buf(&self, out: &mut Grid) {
        *out = self
            .grid
            .clone()
            .center()
            .subtract(&Grid::ROWS[self.state.current].clone());
    }
}

impl Rasterization<Active> for Board<ProcessesRows> {
    fn rasterize_buf(&self, out: &mut Grid) {
        *out = self
            .grid
            .clone()
            .center()
            .intersect(&Grid::ROWS[self.state.current].clone());
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

        // Two rows are fully populated, hence we have to call `process_row` BOARD_ROWS + 2 - 1 times.
        // The last call to `process_row` will produce an Either::right value
        for iter in 1..(BOARD_ROWS + 2) {
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
