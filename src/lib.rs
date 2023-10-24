#![no_std]

use core::marker::PhantomData;

pub type Board<const M: usize, const N: usize> = [[bool; N]; M]; // row major encoding

/// 1x1 square
pub struct Square {}

/// 2x2 diagonal line
pub struct Diagonal {}

/// 2x1 vertical line
pub struct Line {}

pub struct Zero {}
pub struct Ninety {}
pub struct OneEighty {}

pub struct TwoSeventy {}

pub trait Set {
    fn covers(&self, x: i32, y: i32) -> bool;
}

impl Set for Square {
    fn covers(&self, x: i32, y: i32) -> bool {
        x == 0 && y == 0
    }
}

impl Set for Diagonal {
    fn covers(&self, x: i32, y: i32) -> bool {
        (x == 0 && y == 0) || (x == 1 && y == 1)
    }
}

impl Set for Line {
    fn covers(&self, x: i32, y: i32) -> bool {
        x == 0 && (y == 0 || y == 1)
    }
}

pub struct RotatedTile<T, R> {
    t: T,
    r: PhantomData<R>,
}

impl<T> RotatedTile<T, Zero> {
    pub fn new(t: T) -> Self {
        Self { t, r: PhantomData }
    }
}

impl<T, R> Rotatee for RotatedTile<T, R>
where
    R: Rotatee,
{
    type Rotated = RotatedTile<T, R::Rotated>;

    fn rotate(self) -> RotatedTile<T, R::Rotated> {
        RotatedTile {
            t: self.t,
            r: PhantomData,
        }
    }
}

impl<T> Set for RotatedTile<T, Zero>
where
    T: Set,
{
    fn covers(&self, x: i32, y: i32) -> bool {
        self.t.covers(x, y)
    }
}

impl<T> Set for RotatedTile<T, Ninety>
where
    T: Set,
{
    fn covers(&self, x: i32, y: i32) -> bool {
        self.t.covers(-y, x)
    }
}

impl<T> Set for RotatedTile<T, OneEighty>
where
    T: Set,
{
    fn covers(&self, x: i32, y: i32) -> bool {
        self.t.covers(-x, -y)
    }
}

impl<T> Set for RotatedTile<T, TwoSeventy>
where
    T: Set,
{
    fn covers(&self, x: i32, y: i32) -> bool {
        self.t.covers(y, -x)
    }
}

pub struct DisplacedTile<T> {
    t: T,
    displ_x: i32,
    displ_y: i32,
}

impl<T> DisplacedTile<T> {
    pub fn new(t: T) -> Self {
        Self {
            t,
            displ_x: 0,
            displ_y: 0,
        }
    }

    pub fn displace_by(self, x: i32, y: i32) -> Self {
        Self {
            t: self.t,
            displ_x: self.displ_x + x,
            displ_y: self.displ_y + y,
        }
    }
}

impl<T> Rotatee for DisplacedTile<T>
where
    T: Rotatee,
{
    type Rotated = DisplacedTile<T::Rotated>;

    fn rotate(self) -> Self::Rotated {
        DisplacedTile {
            t: self.t.rotate(),
            displ_x: self.displ_x,
            displ_y: self.displ_y,
        }
    }
}

impl<T> Set for DisplacedTile<T>
where
    T: Set,
{
    fn covers(&self, x: i32, y: i32) -> bool {
        self.t.covers(x - self.displ_x, y - self.displ_y)
    }
}

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
                board[row][col] = self.covers(col as i32, row as i32);
            }
        }
    }
}

pub trait Rotatee {
    type Rotated;

    fn rotate(self) -> Self::Rotated;
}

impl Rotatee for Zero {
    type Rotated = Ninety;

    fn rotate(self) -> Self::Rotated {
        Self::Rotated {}
    }
}

impl Rotatee for Ninety {
    type Rotated = OneEighty;

    fn rotate(self) -> Self::Rotated {
        Self::Rotated {}
    }
}

impl Rotatee for OneEighty {
    type Rotated = TwoSeventy;

    fn rotate(self) -> Self::Rotated {
        Self::Rotated {}
    }
}

impl Rotatee for TwoSeventy {
    type Rotated = Zero;

    fn rotate(self) -> Self::Rotated {
        Self::Rotated {}
    }
}

pub enum Tile {
    Square(Square),
    Diagonal(Diagonal),
    Line(Line),
}

impl Set for Tile {
    fn covers(&self, x: i32, y: i32) -> bool {
        match self {
            Tile::Square(s) => s.covers(x, y),
            Tile::Diagonal(d) => d.covers(x, y),
            Tile::Line(l) => l.covers(x, y),
        }
    }
}

pub enum Angle {
    Zero(Zero),
    Ninety(Ninety),
    OneEighty(OneEighty),
    TwoSeventy(TwoSeventy),
}

impl Rotatee for Angle {
    type Rotated = Self;

    fn rotate(self) -> Self::Rotated {
        match self {
            Angle::Zero(z) => Angle::Ninety(Ninety {}),
            Angle::Ninety(n) => Angle::OneEighty(OneEighty {}),
            Angle::OneEighty(o) => Angle::TwoSeventy(TwoSeventy {}),
            Angle::TwoSeventy(t) => Angle::Zero(Zero {}),
        }
    }
}

struct Game<const M: usize, const N: usize> {
    tile: Option<DisplacedTile<RotatedTile<Tile, Angle>>>,
    board: Board<M, N>,
}

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
