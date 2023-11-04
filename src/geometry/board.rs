use array_init::array_init;

use super::raster::Rasterization;

// As of now, computations on const generics are not possible in a somewhat stable manner
// There is [`feature(generic_const_exprs)`](https://github.com/rust-lang/rust/issues/76560), but
// that is _very_ experimental still.
// So instead of having everything nice and generic, I'll hardcode the dimensions of a board.
pub const BOARD_ROWS: usize = 5;
pub const BOARD_COLS: usize = 5;

pub struct Board([[bool; BOARD_COLS + 2]; BOARD_ROWS + 2]); // row major encoding

impl Board {
    #[must_use]
    pub fn new() -> Self {
        let rim = array_init(|r| {
            if r == 0 || r == BOARD_ROWS + 1 {
                [true; BOARD_COLS + 2]
            } else {
                array_init(|c| c == 0 || c == BOARD_COLS + 1)
            }
        });
        Self { 0: rim }
    }
}

impl Default for Board {
    #[must_use]
    fn default() -> Self {
        Self::new()
    }
}

impl Rasterization<{ BOARD_ROWS + 2 }, { BOARD_COLS + 2 }> for Board {
    fn rasterize(&self) -> [[bool; BOARD_ROWS + 2]; BOARD_COLS + 2] {
        self.0.clone()
    }
}
