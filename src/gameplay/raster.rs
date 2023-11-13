// TODO: to be moved to gameplay module

use crate::geometry::grid::Grid;

pub struct Active;
pub struct Passive;

pub trait Rasterization<Role> {
    fn rasterize_buf(&self, out: &mut Grid);
}

pub trait RasterizationExt<Role> {
    fn rasterize(&self) -> Grid;
}

impl<Role, T> RasterizationExt<Role> for T
where
    T: Rasterization<Role>,
{
    fn rasterize(&self) -> Grid {
        let mut grid: Grid;
        self.rasterize_buf(&mut grid);
        grid
    }
}
