pub trait Point<T> {
    fn pt(&self) -> Pt<T> {
        Pt::new(self.x(), self.y())
    }
    fn tuple(&self) -> (T, T) {
        (self.x(), self.y())
    }
    fn x(&self) -> T;
    fn y(&self) -> T;
}

impl<T> Point<T> for (T, T)
where
    T: Copy,
{
    fn x(&self) -> T {
        self.0
    }
    fn y(&self) -> T {
        self.1
    }
}

impl<T> Point<T> for Pt<T>
where
    T: Copy,
{
    fn x(&self) -> T {
        self.x()
    }
    fn y(&self) -> T {
        self.y()
    }
}

#[derive(Clone, Debug)]
pub struct Pt<T> {
    pub x: T,
    pub y: T,
}

impl<T> Copy for Pt<T> where T: Copy {}

impl<T> Pt<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn x(&self) -> T
    where
        T: Copy,
    {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> T
    where
        T: Copy,
    {
        self.y
    }

    pub fn add(&self, value: T) -> Self
    where
        T: Copy + std::ops::Add<Output = T>,
    {
        Self {
            x: self.x + value,
            y: self.y + value,
        }
    }

    pub fn sub(&self, value: T) -> Self
    where
        T: Copy + std::ops::Sub<Output = T>,
    {
        Self {
            x: self.x - value,
            y: self.y - value,
        }
    }

    pub fn mul(&self, value: T) -> Self
    where
        T: Copy + std::ops::Mul<Output = T>,
    {
        Self {
            x: self.x * value,
            y: self.y * value,
        }
    }

    pub fn div(&self, value: T) -> Self
    where
        T: Copy + std::ops::Div<Output = T>,
    {
        Self {
            x: self.x / value,
            y: self.y / value,
        }
    }

    pub(crate) fn real_to_iter(mut self, oct: u8, c: Pt<T>) -> Pt<T>
    where
        T: Copy + std::ops::Neg<Output = T> + std::ops::SubAssign,
    {
        self.x -= c.x();
        self.y -= c.y();
        match oct {
            1 => Pt::new(-self.y, self.x),
            2 => Pt::new(self.x, -self.y),
            3 => Pt::new(-self.x, -self.y),
            4 => Pt::new(-self.y, -self.x),
            5 => Pt::new(self.y, -self.x),
            6 => Pt::new(-self.x, self.y),
            7 => Pt::new(self.x, self.y),
            8 => Pt::new(self.y, self.x),
            _ => Pt::new(self.x, self.y),
        }
    }

    pub(crate) fn iter_to_quad(&self, quad: u8, c: Pt<T>) -> Self
    where
        T: Copy + std::ops::Add<Output = T> + std::ops::Neg<Output = T>,
    {
        match quad {
            1 => Pt::new(self.y + c.x(), -self.x + c.y()),
            2 => Pt::new(-self.x + c.x(), -self.y + c.y()),
            3 => Pt::new(-self.y + c.x(), self.x + c.y()),
            4 => Pt::new(self.x + c.x(), self.y + c.y()),
            _ => panic!("invalid quadrant"),
        }
    }

    pub(crate) fn quad_to_iter(&self, quad: u8, c: Pt<T>) -> Self
    where
        T: Copy + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + std::fmt::Debug,
    {
        let x = self.x - c.x();
        let y = self.y - c.y();
        match quad {
            1 => Pt::new(-y, x),
            2 => Pt::new(-x, -y),
            3 => Pt::new(y, -x),
            4 => Pt::new(x, y),
            _ => panic!("invalid quadrant"),
        }
    }
}

impl Pt<f64> {
    pub fn from_radian<T>(angle: f64, radius: T, center: (T, T)) -> Self
    where
        T: Into<f64> + Copy,
    {
        let x = center.0.into() + radius.into() * angle.cos();
        let y = center.1.into() - radius.into() * angle.sin();

        Self { x, y }
    }

    pub fn i32(&self) -> Pt<i32> {
        Pt {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }

    #[allow(dead_code)]
    pub fn u32(&self) -> Pt<u32> {
        Pt {
            x: self.x.abs().round() as u32,
            y: self.y.abs().round() as u32,
        }
    }
}

impl Pt<i32> {
    #[cfg(test)]
    #[allow(dead_code)]
    pub(super) const fn iter_to_real(self, oct: u8, c: Pt<i32>) -> Pt<i32> {
        match oct {
            1 => Pt::new(self.y + c.x(), -self.x + c.y()),
            2 => Pt::new(self.x + c.x(), -self.y + c.y()),
            3 => Pt::new(-self.x + c.x(), -self.y + c.y()),
            4 => Pt::new(-self.y + c.x(), -self.x + c.y()),
            5 => Pt::new(-self.y + c.x(), self.x + c.y()),
            6 => Pt::new(-self.x + c.x(), self.y + c.y()),
            7 => Pt::new(self.x + c.x(), self.y + c.y()),
            8 => Pt::new(self.y + c.x(), self.x + c.y()),
            _ => Pt::new(self.x + c.x(), self.y + c.y()),
        }
    }

    pub fn u32(&self) -> Pt<u32> {
        Pt {
            x: self.x as u32,
            y: self.y as u32,
        }
    }

    pub fn abs_u32(&self) -> Pt<u32> {
        Pt {
            x: self.x.unsigned_abs(),
            y: self.y.unsigned_abs(),
        }
    }

    pub fn is_negative(&self) -> bool {
        self.x.is_negative() | self.y.is_negative()
    }
}

impl Pt<u32> {
    pub fn i32(&self) -> Pt<i32> {
        Pt {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    pub fn f32(&self) -> Pt<f32> {
        Pt {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    pub fn f64(&self) -> Pt<f64> {
        Pt {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

impl<T> From<(T, T)> for Pt<T> {
    fn from(tuple: (T, T)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl<T> From<Pt<T>> for (T, T) {
    fn from(pt: Pt<T>) -> Self {
        (pt.x, pt.y)
    }
}

impl From<Pt<i32>> for Pt<f64> {
    fn from(pt: Pt<i32>) -> Self {
        Self {
            x: pt.x.into(),
            y: pt.y.into(),
        }
    }
}

impl From<Pt<u32>> for Pt<i32> {
    fn from(pt: Pt<u32>) -> Self {
        Self {
            x: pt.x as i32,
            y: pt.y as i32,
        }
    }
}

impl From<Pt<u32>> for Pt<f32> {
    fn from(pt: Pt<u32>) -> Self {
        Self {
            x: pt.x as f32,
            y: pt.y as f32,
        }
    }
}

impl From<Pt<f64>> for Pt<i32> {
    fn from(pt: Pt<f64>) -> Self {
        Self {
            x: pt.x.round() as i32,
            y: pt.y.round() as i32,
        }
    }
}

impl<T> std::ops::Add for Pt<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> std::ops::Sub for Pt<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> std::ops::Mul for Pt<T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> std::ops::Div for Pt<T>
where
    T: std::ops::Div<Output = T>,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
