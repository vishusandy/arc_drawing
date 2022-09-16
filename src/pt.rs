#[derive(Clone, Debug)]
pub struct Pt<T> {
    pub(crate) x: T,
    pub(crate) y: T,
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
        #[cfg(test)]
        log::debug!("Quad to iter: {:?}", self);
        let x = self.x - c.x();
        let y = self.y - c.y();
        match quad {
            1 => Pt::new(-y, x),
            2 => Pt::new(-x, -y),
            // 3 => Pt::new(-x, y),
            3 => Pt::new(y, -x),
            4 => Pt::new(x, y),
            _ => panic!("invalid quadrant"),
        }
    }
}

impl Pt<f64> {
    pub(crate) fn from_radian<T>(angle: f64, radius: T, center: (T, T)) -> Self
    where
        T: Into<f64> + Copy,
    {
        let x = center.0.into() + radius.into() * angle.cos();
        let y = center.1.into() - radius.into() * angle.sin();

        Self { x, y }
    }

    #[inline]
    pub(crate) fn i32(&self) -> Pt<i32> {
        Pt {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn u32(&self) -> Pt<u32> {
        Pt {
            x: self.x.abs().round() as u32,
            y: self.y.abs().round() as u32,
        }
    }
}

impl Pt<i32> {
    #[cfg(test)]
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

    #[inline]
    pub(crate) fn u32(&self) -> Pt<u32> {
        Pt {
            x: self.x as u32,
            y: self.y as u32,
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
