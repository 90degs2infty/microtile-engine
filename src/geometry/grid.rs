use super::tile::{BasicTile, Discrete2DSet, DisplacedTile, RotatedTile};
use paste::paste;

#[derive(Debug)]
pub enum GridError {
    /// Access using invalid index `(row, col)`
    InvalidIndex(Option<usize>, Option<usize>),
    /// Non-empty set lead to an empty grid representation
    EmptyIntersection,
}

/// 5 by 5 grid encoded in an `u32`
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid(u32);

macro_rules! row {
    ($r:literal, $val:literal) => {
        paste! {
            const [< R $r _RAW >]: u32 = $val;
            pub const [< R $r >]: Self = Self::new(Self::[< R $r _RAW >]);
        }
    };
}

impl Grid {
    pub const NUM_ROWS: usize = 5;
    pub const NUM_COLS: usize = 5;

    row!(0, 0x0000_001f);
    row!(1, 0x0000_03e0);
    row!(2, 0x0000_7c00);
    row!(3, 0x000f_8000);
    row!(4, 0x01f0_0000);

    pub const ROWS: [Self; 5] = [Self::R0, Self::R1, Self::R2, Self::R3, Self::R4];

    const ROWS_BELOW: [u32; 5] = [
        0x0,
        Self::R0_RAW,
        Self::R0_RAW | Self::R1_RAW,
        Self::R0_RAW | Self::R1_RAW | Self::R2_RAW,
        Self::R0_RAW | Self::R1_RAW | Self::R2_RAW | Self::R3_RAW,
    ];

    const ROWS_ABOVE: [u32; 5] = [
        Self::R4_RAW | Self::R3_RAW | Self::R2_RAW | Self::R1_RAW,
        Self::R4_RAW | Self::R3_RAW | Self::R2_RAW,
        Self::R4_RAW | Self::R3_RAW,
        Self::R4_RAW,
        0x0,
    ];

    fn new(grid: u32) -> Self {
        Self { 0: grid }
    }

    // elements are encoded row major
    const fn element_to_bit_idx(row: usize, col: usize) -> Option<usize> {
        if row >= Self::NUM_ROWS || col >= Self::NUM_ROWS {
            None
        } else {
            Some(row * Self::NUM_COLS + col)
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

    pub fn overlaps(&self, other: &Self) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn union(self, other: Self) -> Self {
        Self::new(self.0 | other.0)
    }

    pub fn contains(&self, other: &Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn set_element(self, row: usize, col: usize) -> Result<Self, GridError> {
        Self::element_bit(row, col)
            .map(|bit| Self::new(self.0 | bit))
            .ok_or(GridError::InvalidIndex(Some(row), Some(col)))
    }

    pub fn clear_element(self, row: usize, col: usize) -> Result<Self, GridError> {
        Self::element_bit(row, col)
            .map(|bit| Self::new(self.0 & !bit))
            .ok_or(GridError::InvalidIndex(Some(row), Some(col)))
    }

    pub fn is_element_set(&self, row: usize, col: usize) -> Result<bool, GridError> {
        Self::element_bit(row, col)
            .map(|bit| (self.0 & bit) != 0)
            .ok_or(GridError::InvalidIndex(Some(row), Some(col)))
    }

    /// Discard specified row and shift all rows above downwards by one row
    pub fn discard_and_shift(self, row: usize) -> Result<Self, GridError> {
        if row >= Self::NUM_ROWS {
            return Err(GridError::InvalidIndex(Some(row), None));
        }

        let above = self.0 & Self::ROWS_ABOVE[row];
        let below = self.0 & Self::ROWS_BELOW[row];

        Ok(Self::new((above >> Self::NUM_COLS) | below))
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<[[bool; Self::NUM_COLS]; Self::NUM_ROWS]> for Grid {
    fn from(value: [[bool; Self::NUM_COLS]; Self::NUM_ROWS]) -> Self {
        let mut grid = Self::default();

        for row in 0..Self::NUM_ROWS {
            for col in 0..Self::NUM_COLS {
                if value[row][col] {
                    grid = grid
                        .set_element(row, col)
                        .expect("Hardcoded range should be valid");
                } else {
                    grid = grid
                        .clear_element(row, col)
                        .expect("Hardcoded range should be valid");
                }
            }
        }

        grid
    }
}

/// 7 by 7 grid encoded in an `u64`
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExtGrid(u64);

impl ExtGrid {
    pub const NUM_ROWS: usize = Grid::NUM_ROWS + 2;
    pub const NUM_COLS: usize = Grid::NUM_COLS + 2;

    // rim is encoded in upper half of u64
    const OFFSET_BOTTOM_EDGE: usize = 32;
    const OFFSET_FIRST_CENTER_EDGE: usize = Self::OFFSET_BOTTOM_EDGE + Self::NUM_COLS;
    const OFFSET_TOP_EDGE: usize = Self::OFFSET_BOTTOM_EDGE + 2 * (Self::NUM_ROWS - 2);

    const TOP_ROW_IDX: usize = Self::NUM_ROWS - 1;
    const RIGHT_COL_IDX: usize = Self::NUM_COLS - 1;

    const RIM_RAW: u64 = 0xffffff00000000;
    pub const RIM: Self = Self::new(Self::RIM_RAW);

    const fn new(grid: u64) -> Self {
        Self { 0: grid }
    }

    // bit indices for the "inner" (that is not the corners) part of vertical edges
    const fn vertical_rim_element_to_bit_idx(row: usize, col: usize) -> Option<usize> {
        if row > 0 && row < Self::TOP_ROW_IDX {
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
                if col < Self::NUM_COLS {
                    Some(Self::OFFSET_BOTTOM_EDGE + col)
                } else {
                    None
                }
            }
            // top edge
            (Self::TOP_ROW_IDX, col) => {
                if col < Self::NUM_COLS {
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

    pub fn overlaps(&self, other: &Self) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn union(self, other: Self) -> Self {
        Self::new(self.0 | other.0)
    }

    pub fn contains(&self, other: &Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn set_element(self, row: usize, col: usize) -> Result<Self, GridError> {
        Self::element_bit(row, col)
            .map(|bit| Self::new(self.0 | bit))
            .ok_or(GridError::InvalidIndex(Some(row), Some(col)))
    }

    pub fn clear_element(self, row: usize, col: usize) -> Result<Self, GridError> {
        Self::element_bit(row, col)
            .map(|bit| Self::new(self.0 & !bit))
            .ok_or(GridError::InvalidIndex(Some(row), Some(col)))
    }

    pub fn is_element_set(&self, row: usize, col: usize) -> Result<bool, GridError> {
        Self::element_bit(row, col)
            .map(|bit| (self.0 & bit) != 0)
            .ok_or(GridError::InvalidIndex(Some(row), Some(col)))
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

impl From<&Grid> for ExtGrid {
    fn from(value: &Grid) -> Self {
        Self::new(value.0 as u64)
    }
}

impl From<[[bool; Self::NUM_COLS]; Self::NUM_ROWS]> for ExtGrid {
    fn from(value: [[bool; Self::NUM_COLS]; Self::NUM_ROWS]) -> Self {
        let mut grid = Self::default();

        for row in 0..(Self::NUM_ROWS) {
            for col in 0..(Self::NUM_COLS) {
                if value[row][col] {
                    grid = grid
                        .set_element(row, col)
                        .expect("Hardcoded range should be valid");
                } else {
                    grid = grid
                        .clear_element(row, col)
                        .expect("Hardcoded range should be valid");
                }
            }
        }

        grid
    }
}

/*
 * I'd love to do this
 *
 * ```rust
 * impl<T> TryFrom<T> for ExtGrid
 * where
 *     T: Discrete2DSet,
 * {
 *     type Error = GridError;
 *     fn try_from(value: T) -> Result<Self, Self::Error> {
 *         let mut grid = ExtGrid::default();
 *
 *         for row in 0..(Self::NUM_ROWS) {
 *             for col in 0..(Self::NUM_COLS) {
 *                 if value.contains(col, row) {
 *                     grid = grid
 *                         .set_element(row, col)
 *                         .expect("Hardcoded range should be valid")
 *                 }
 *             }
 *         }
 *
 *         if !value.is_empty() && grid.is_empty() {
 *             Err(GridError::EmptyIntersection)
 *         }
 *
 *         Ok(grid)
 *     }
 * }
 * ```
 *
 * but this gives:
 *
 * ```text
 *
 * error[E0119]: conflicting implementations of trait `TryFrom<_>` for type `ExtGrid`
 *    --> src/geometry/grid.rs:257:1
 *     |
 * 257 | impl<T> TryFrom<T> for ExtGrid
 *     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *     |
 *     = note: conflicting implementation in crate `core`:
 *             - impl<T, U> TryFrom<U> for T
 *               where U: Into<T>;
 *
 * because of https://users.rust-lang.org/t/trait-bounds-limitations-generic-tryfrom-t-impl-hypothetical-future-impl-triggers-conflicting-implementations-error/101900/1
 *
 * So instead, I have to give more specific impls for each type.
 */
impl ExtGrid {
    fn try_from_discrete_2d_set<T>(value: &T) -> Result<Self, GridError>
    where
        T: Discrete2DSet,
    {
        let mut grid = ExtGrid::default();

        for row in 0..(Self::NUM_ROWS) {
            for col in 0..(Self::NUM_COLS) {
                if value.contains(
                    col.try_into().expect("Hardcoded range should be valid"),
                    row.try_into().expect("Hardcoded range should be valid"),
                ) {
                    grid = grid
                        .set_element(row, col)
                        .expect("Hardcoded range should be valid")
                }
            }
        }

        if !value.is_empty() && grid.is_empty() {
            return Err(GridError::EmptyIntersection);
        }

        Ok(grid)
    }
}

// TODO: write a macro for the following impls
impl TryFrom<BasicTile> for ExtGrid {
    type Error = GridError;

    fn try_from(value: BasicTile) -> Result<Self, Self::Error> {
        Self::try_from_discrete_2d_set(&value)
    }
}

impl TryFrom<&BasicTile> for ExtGrid {
    type Error = GridError;

    fn try_from(value: &BasicTile) -> Result<Self, Self::Error> {
        Self::try_from_discrete_2d_set(value)
    }
}

impl<T> TryFrom<RotatedTile<T>> for ExtGrid
where
    T: Discrete2DSet,
{
    type Error = GridError;

    fn try_from(value: RotatedTile<T>) -> Result<Self, Self::Error> {
        Self::try_from_discrete_2d_set(&value)
    }
}

impl<T> TryFrom<&RotatedTile<T>> for ExtGrid
where
    T: Discrete2DSet,
{
    type Error = GridError;

    fn try_from(value: &RotatedTile<T>) -> Result<Self, Self::Error> {
        Self::try_from_discrete_2d_set(value)
    }
}

impl<T> TryFrom<DisplacedTile<T>> for ExtGrid
where
    T: Discrete2DSet,
{
    type Error = GridError;

    fn try_from(value: DisplacedTile<T>) -> Result<Self, Self::Error> {
        Self::try_from_discrete_2d_set(&value)
    }
}

impl<T> TryFrom<&DisplacedTile<T>> for ExtGrid
where
    T: Discrete2DSet,
{
    type Error = GridError;

    fn try_from(value: &DisplacedTile<T>) -> Result<Self, Self::Error> {
        Self::try_from_discrete_2d_set(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_no_element_set() -> Result<(), GridError> {
        let grid = Grid::default();

        for row in 0..Grid::NUM_ROWS {
            for col in 0..Grid::NUM_COLS {
                assert!(!grid.is_element_set(row, col)?)
            }
        }
        Ok(())
    }

    #[test]
    fn set_element_consistent_with_is_element_set() -> Result<(), GridError> {
        for row in 0..Grid::NUM_ROWS {
            for col in 0..Grid::NUM_COLS {
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

        for row in 0..ExtGrid::NUM_ROWS {
            for col in 0..ExtGrid::NUM_COLS {
                assert!(!grid.is_element_set(row, col)?)
            }
        }
        Ok(())
    }

    #[test]
    fn set_element_consistent_with_is_element_set_ext() -> Result<(), GridError> {
        for row in 0..ExtGrid::NUM_ROWS {
            for col in 0..ExtGrid::NUM_COLS {
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

        for row in 1..(ExtGrid::NUM_ROWS - 1) {
            for col in 1..(ExtGrid::NUM_COLS - 1) {
                assert!(!rim.is_element_set(row, col)?);
            }
        }

        for col in 0..ExtGrid::NUM_COLS {
            assert!(rim.is_element_set(0, col)?);
            assert!(rim.is_element_set(ExtGrid::NUM_ROWS - 1, col)?);
        }

        for row in 0..ExtGrid::NUM_ROWS {
            assert!(rim.is_element_set(row, 0)?);
            assert!(rim.is_element_set(row, ExtGrid::NUM_COLS - 1)?);
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
