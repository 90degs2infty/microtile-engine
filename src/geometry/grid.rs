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
