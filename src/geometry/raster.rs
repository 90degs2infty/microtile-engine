pub trait Rasterization<const M: usize, const N: usize> {
    #[must_use]
    fn rasterize(&self) -> [[bool; N]; M];
}
