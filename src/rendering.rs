pub struct Active;

pub struct Passive;

pub trait Rendering<const M: usize, const N: usize, T> {
    fn render_buf(&self, buffer: &mut [[bool; N]; M]);
}

pub trait RenderingExt<const M: usize, const N: usize, T>: Rendering<M, N, T> {
    fn render(&self) -> [[bool; N]; M];
}

impl<const M: usize, const N: usize, R, T> RenderingExt<M, N, T> for R
where
    R: Rendering<M, N, T>,
{
    fn render(&self) -> [[bool; N]; M] {
        let mut buf = [[false; N]; M];
        self.render_buf(&mut buf);
        buf
    }
}
