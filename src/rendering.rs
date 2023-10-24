use crate::geometry::{board::Board, tile::Set};

pub trait Render {
    fn render<const M: usize, const N: usize>(&self, board: &mut Board<M, N>);
}

impl<T> Render for T
where
    T: Set,
{
    fn render<const M: usize, const N: usize>(&self, board: &mut Board<M, N>) {
        for (r, row) in board.iter_mut().enumerate() {
            for (c, square) in row.iter_mut().enumerate() {
                // x corresponds to columns and y corresponds to rows!
                *square = self.contains(c as i32, r as i32);
            }
        }
    }
}
