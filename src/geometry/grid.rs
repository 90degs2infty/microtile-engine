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
pub struct ExtGrid(u64);

impl ExtGrid {
    pub const RIM: Self = Self::new(0);

    const fn new(grid: u64) -> Self {
        Self { 0: grid }
    }

    pub fn overlaps(&self, other: &ExtGrid) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn set_element(self, row: usize, col: usize) -> Result<Self, GridError> {
        todo!()
    }

    pub fn is_element_set(&self, row: usize, col: usize) -> Result<bool, GridError> {
        todo!()
    }

    pub fn center(self) -> Grid {
        todo!()
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
}
