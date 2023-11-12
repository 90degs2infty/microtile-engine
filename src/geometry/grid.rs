use super::board::{BOARD_COLS, BOARD_ROWS};

#[derive(Debug)]
pub enum GridError {
    InvalidIndex,
}

/// 5 by 5 grid encoded in an `u32`
#[derive(Debug, PartialEq, Eq)]
pub struct Grid(u32);

impl Grid {
    fn new(grid: u32) -> Self {
        Self { 0: grid }
    }

    // elements are encoded row major
    const fn element_to_bit_idx(row: usize, col: usize) -> Option<usize> {
        if row >= BOARD_ROWS || col >= BOARD_COLS {
            None
        } else {
            Some(row * BOARD_COLS + col)
        }
    }

    const fn element_bit(row: usize, col: usize) -> Option<u32> {
        // as of now, you cannot call `.map()` on an `Option` from
        // a const function, so we have to rebuild `map` locally
        match Self::element_to_bit_idx(row, col) {
            None => None,
            Some(idx) => Some(1 << idx),
        }
    }

    pub fn overlaps(&self, other: &Grid) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn set_element(self, row: usize, col: usize) -> Result<Self, GridError> {
        Self::element_bit(row, col)
            .map(|bit| Self::new(self.0 | bit))
            .ok_or(GridError::InvalidIndex)
    }

    pub fn is_element_set(&self, row: usize, col: usize) -> Result<bool, GridError> {
        Self::element_bit(row, col)
            .map(|bit| (self.0 & bit) != 0)
            .ok_or(GridError::InvalidIndex)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(0)
    }
}

/// 7 by 7 grid encoded in an `u64`
#[derive(Debug, PartialEq, Eq)]
pub struct ExtGrid(u64);

impl ExtGrid {
    pub const RIM: Self = Self::new(0);
    // rim is encoded in upper half of u64
    const OFFSET_BOTTOM_EDGE: usize = 32;
    const OFFSET_FIRST_CENTER_EDGE: usize = Self::OFFSET_BOTTOM_EDGE + BOARD_COLS + 2;
    const OFFSET_TOP_EDGE: usize = Self::OFFSET_BOTTOM_EDGE + 2 * BOARD_ROWS;

    const TOP_ROW_IDX: usize = BOARD_ROWS + 1;
    const RIGHT_COL_IDX: usize = BOARD_COLS + 1;

    const fn new(grid: u64) -> Self {
        Self { 0: grid }
    }

    // bit indices for the "inner" (that is not the corners) part of vertical edges
    const fn vertical_rim_element_to_bit_idx(row: usize, col: usize) -> Option<usize> {
        if row >= 1 && row <= BOARD_ROWS {
            let offset = Self::OFFSET_FIRST_CENTER_EDGE + (row - 1) * 2;
            match col {
                0 => Some(offset),
                Self::RIGHT_COL_IDX => Some(offset + 1),
                _ => None,
            }
        } else {
            None
        }
    }

    const fn element_to_bit_idx(row: usize, col: usize) -> Option<usize> {
        match (row, col) {
            // bottom edge
            (0, col) => {
                if col < BOARD_COLS + 2 {
                    Some(Self::OFFSET_BOTTOM_EDGE + col)
                } else {
                    None
                }
            }
            // top edge
            (Self::TOP_ROW_IDX, col) => {
                if col < BOARD_COLS + 2 {
                    Some(Self::OFFSET_TOP_EDGE + col)
                } else {
                    None
                }
            }
            // left edge
            (row, 0) => Self::vertical_rim_element_to_bit_idx(row, 0),
            // right edge
            (row, Self::RIGHT_COL_IDX) => {
                Self::vertical_rim_element_to_bit_idx(row, Self::RIGHT_COL_IDX)
            }
            // inner (or outside, but handled the same way)
            (row, col) => Grid::element_to_bit_idx(row - 1, col - 1),
        }
    }

    const fn element_bit(row: usize, col: usize) -> Option<u64> {
        // as of now, you cannot call `.map()` on an `Option` from
        // a const function, so we have to rebuild `map` locally
        match Self::element_to_bit_idx(row, col) {
            Some(val) => Some(1 << val),
            None => None,
        }
    }

    pub fn overlaps(&self, other: &ExtGrid) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn set_element(self, row: usize, col: usize) -> Result<Self, GridError> {
        Self::element_bit(row, col)
            .map(|bit| Self::new(self.0 | bit))
            .ok_or(GridError::InvalidIndex)
    }

    pub fn is_element_set(&self, row: usize, col: usize) -> Result<bool, GridError> {
        Self::element_bit(row, col)
            .map(|bit| (self.0 & bit) != 0)
            .ok_or(GridError::InvalidIndex)
    }

    pub fn center(self) -> Grid {
        // the center part is encoded at the lower half of `self.0`
        Grid::new((self.0 & 0xffffffff) as u32)
    }
}

impl Default for ExtGrid {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<Grid> for ExtGrid {
    fn from(value: Grid) -> Self {
        Self::new(value.0 as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_no_element_set() -> Result<(), GridError> {
        let grid = Grid::default();

        for row in 0..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                assert!(!grid.is_element_set(row, col)?)
            }
        }
        Ok(())
    }

    #[test]
    fn set_element_consistent_with_is_element_set() -> Result<(), GridError> {
        for row in 0..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                assert!(Grid::default()
                    .set_element(row, col)?
                    .is_element_set(row, col)?);
            }
        }
        Ok(())
    }

    #[test]
    fn default_no_element_set_ext() -> Result<(), GridError> {
        let grid = ExtGrid::default();

        for row in 0..(BOARD_ROWS + 2) {
            for col in 0..(BOARD_COLS + 2) {
                assert!(!grid.is_element_set(row, col)?)
            }
        }
        Ok(())
    }

    #[test]
    fn set_element_consistent_with_is_element_set_ext() -> Result<(), GridError> {
        for row in 0..(BOARD_ROWS + 2) {
            for col in 0..(BOARD_COLS + 2) {
                assert!(ExtGrid::default()
                    .set_element(row, col)?
                    .is_element_set(row, col)?);
            }
        }
        Ok(())
    }

    #[test]
    fn rim() -> Result<(), GridError> {
        let rim = ExtGrid::RIM;

        for row in 0..BOARD_ROWS {
            for col in 0..BOARD_COLS {
                assert!(!rim.is_element_set(1 + row, 1 + col)?);
            }
        }

        for col in 0..(BOARD_COLS + 2) {
            assert!(rim.is_element_set(0, col)?);
            assert!(rim.is_element_set(BOARD_ROWS + 1, col)?);
        }

        for row in 0..BOARD_ROWS {
            assert!(rim.is_element_set(row + 1, 0)?);
            assert!(rim.is_element_set(row + 1, BOARD_COLS + 1)?);
        }
        Ok(())
    }

    #[test]
    fn ext_center_matches_grid() -> Result<(), GridError> {
        let grid = Grid::default()
            .set_element(0, 0)?
            .set_element(1, 1)?
            .set_element(2, 2)?;
        let ext_grid = ExtGrid::default()
            .set_element(1, 1)?
            .set_element(2, 2)?
            .set_element(3, 3)?;

        assert_eq!(grid, ext_grid.center());
        Ok(())
    }
}
