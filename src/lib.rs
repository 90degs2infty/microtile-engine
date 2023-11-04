#![no_std]

pub mod gameplay;
pub mod geometry;
pub mod rendering;

/*

struct Tile<T, R> {
    t: PhantomData<T>,
    r: PhantomData<R>,
    displ_x: i16,
    displ_y: i16,
}

impl<T, R> Tile<T, R>
where
    R: Rotatee,
{
    pub fn rotate(self) -> Tile<T, R::Rotated> {
        Tile {
            t: PhantomData::<T> {},
            r: PhantomData::<R::Rotated> {},
            displ_x: self.displ_x,
            displ_y: self.displ_y,
        }
    }

    fn covers(&self, row: usize, col: usize) -> bool {
        false
    }

    pub fn render<const M: usize, const N: usize>(&self, board: &mut Board<M, N>) {
        for row in 0..M {
            for col in 0..N {
                board[row][col] = self.covers(row, col);
            }
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

*/
