use super::board::{BOARD_COLS, BOARD_ROWS};

#[derive(Debug)]
pub enum GridError {
    InvalidIndex,
}

/// 5 by 5 grid encoded in an `u32`
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
        todo!()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_no_element_set() -> Result<(), GridError> {
        let grid = Grid::default();

        for row in 0..5 {
            for col in 0..5 {
                assert!(!grid.is_element_set(row, col)?)
            }
        }
        Ok(())
    }

    #[test]
    fn set_element_consistent_with_is_element_set() -> Result<(), GridError> {
        for row in 0..5 {
            for col in 0..5 {
                assert!(Grid::default()
                    .set_element(row, col)?
                    .is_element_set(row, col)?);
            }
        }
        Ok(())
    }
}
