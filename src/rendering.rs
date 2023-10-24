use crate::geometry::{board::Board, tile::Set};

pub trait Render {
    fn render<const M: usize, const N: usize>(&self, board: &mut Board<M, N>);
}

impl<T> Render for T
where
    T: Set,
{
    fn render<const M: usize, const N: usize>(&self, board: &mut Board<M, N>) {
        for row in 0..M {
            for col in 0..N {
                // x corresponds to columns and y corresponds to rows!
                board[row][col] = self.contains(col as i32, row as i32);
            }
        }
    }
}
