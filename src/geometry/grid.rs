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

    pub fn overlaps(&self, other: &Grid) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn set_element(self, row: usize, col: usize) -> Result<Self, GridError> {
        todo!()
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
