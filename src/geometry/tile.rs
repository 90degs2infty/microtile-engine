use array_init::array_init;

use super::raster::Rasterization;

#[derive(Clone)]
pub enum Angle {
    /// 0° in counter-clockwise order
    Zero,
    /// 90° in counter-clockwise order
    Ninety,
    /// 180° in counter-clockwise order
    OneEighty,
    /// 270° in counter-clockwise order
    TwoSeventy,
}

#[derive(Clone)]
pub enum BasicTile {
    /// 1x1 square
    Square,
    /// 2x2 diagonal line
    Diagonal,
    /// 2x1 vertical line
    Line,
}

#[derive(Clone)]
pub struct RotatedTile<T> {
    t: T,
    a: Angle,
}

impl<T> RotatedTile<T> {
    pub fn new(t: T) -> Self {
        Self { t, a: Angle::Zero }
    }
}

#[derive(Clone)]
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

    pub fn displ_x(&self) -> &i32 {
        &self.displ_x
    }

    pub fn displ_y(&self) -> &i32 {
        &self.displ_y
    }
}

pub trait Discrete2DSet {
    fn contains(&self, x: i32, y: i32) -> bool;
}

impl Discrete2DSet for BasicTile {
    fn contains(&self, x: i32, y: i32) -> bool {
        match self {
            BasicTile::Square => x == 0 && y == 0,
            BasicTile::Diagonal => (x == 0 && y == 0) || (x == 1 && y == 1),
            BasicTile::Line => x == 0 && (y == 0 || y == 1),
        }
    }
}

impl<T> Discrete2DSet for RotatedTile<T>
where
    T: Discrete2DSet,
{
    fn contains(&self, x: i32, y: i32) -> bool {
        // The tile is expected to be rotated counter-clockwise (since the
        // `Rotatee` trait defines rotations to be counter-clockwise).
        // When calling `contains` on the contained tile below, the specified
        // coordinates are interpreted by the tile in its local system of coordinates.
        // Hence, we have to use the clockwise rotation matrix to map the vector
        // (x, y) into the tile-local system of coordinates.
        match self.a {
            Angle::Zero => self.t.contains(x, y),
            Angle::Ninety => self.t.contains(y, -x),
            Angle::OneEighty => self.t.contains(-x, -y),
            Angle::TwoSeventy => self.t.contains(-y, x),
        }
    }
}

impl<T> Discrete2DSet for DisplacedTile<T>
where
    T: Discrete2DSet,
{
    fn contains(&self, x: i32, y: i32) -> bool {
        self.t.contains(x - self.displ_x, y - self.displ_y)
    }
}

pub trait Rotatee {
    type Rotated;

    /// Rotate by 90 degrees in counter-clockwise order
    fn rotate_ccw(self) -> Self::Rotated;
}

impl Rotatee for Angle {
    type Rotated = Self;

    fn rotate_ccw(self) -> Self {
        match self {
            Angle::Zero => Angle::Ninety,
            Angle::Ninety => Angle::OneEighty,
            Angle::OneEighty => Angle::TwoSeventy,
            Angle::TwoSeventy => Angle::Zero,
        }
    }
}

impl Rotatee for BasicTile {
    type Rotated = RotatedTile<BasicTile>;

    fn rotate_ccw(self) -> Self::Rotated {
        Self::Rotated::new(self).rotate_ccw()
    }
}

impl<T> Rotatee for RotatedTile<T> {
    type Rotated = Self;

    fn rotate_ccw(self) -> Self {
        Self {
            t: self.t,
            a: self.a.rotate_ccw(),
        }
    }
}

impl<T> Rotatee for DisplacedTile<T>
where
    T: Rotatee,
{
    type Rotated = DisplacedTile<T::Rotated>;

    fn rotate_ccw(self) -> Self::Rotated {
        Self::Rotated {
            t: self.t.rotate_ccw(),
            displ_x: self.displ_x,
            displ_y: self.displ_y,
        }
    }
}

pub trait Displacee {
    type Displaced;

    fn displace_by(self, x: i32, y: i32) -> Self::Displaced;
}

impl Displacee for BasicTile {
    type Displaced = DisplacedTile<Self>;

    fn displace_by(self, x: i32, y: i32) -> Self::Displaced {
        Self::Displaced::new(self).displace_by(x, y)
    }
}

impl<T> Displacee for RotatedTile<T> {
    type Displaced = DisplacedTile<Self>;

    fn displace_by(self, x: i32, y: i32) -> Self::Displaced {
        Self::Displaced::new(self).displace_by(x, y)
    }
}

impl<T> Displacee for DisplacedTile<T> {
    type Displaced = Self;

    fn displace_by(self, x: i32, y: i32) -> Self {
        Self {
            t: self.t,
            displ_x: self.displ_x + x,
            displ_y: self.displ_y + y,
        }
    }
}

pub trait Dimensionee {
    /// (width, height)
    fn dimensions(&self) -> (usize, usize);
}

impl Dimensionee for BasicTile {
    fn dimensions(&self) -> (usize, usize) {
        match self {
            BasicTile::Square => (1, 1),
            BasicTile::Line => (1, 2),
            BasicTile::Diagonal => (2, 2),
        }
    }
}

impl<const M: usize, const N: usize, T> Rasterization<M, N> for T
where
    T: Discrete2DSet,
{
    fn rasterize(&self) -> [[bool; N]; M] {
        array_init(|r| array_init(|c| self.contains(c as i32, r as i32)))
    }
}
