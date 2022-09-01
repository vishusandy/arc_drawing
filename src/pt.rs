#[derive(Clone, Debug)]
pub(crate) struct Pt<T> {
    x: T,
    y: T,
}
impl<T> Pt<T> {
    pub(crate) fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub(crate) fn x(&self) -> T
    where
        T: Copy,
    {
        self.x
    }
    pub(crate) fn y(&self) -> T
    where
        T: Copy,
    {
        self.y
    }
}
impl Pt<f64> {
    pub(crate) fn from_radian<T>(angle: f64, center: (T, T), radius: T) -> Self
    where
        T: Into<f64> + Copy,
    {
        let x = center.0.into() + radius.into() * angle.cos();
        let y = center.1.into() + radius.into() * angle.sin();

        Self { x, y }
    }
}
