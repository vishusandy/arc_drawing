#[derive(Clone, Debug)]
pub(crate) struct Polar<T> {
    radius: T,
    angle: T,
}

#[derive(Clone, Debug)]
pub struct Pt<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Copy for Pt<T> where T: Copy {}

impl<T> Pt<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> T
    where
        T: Copy,
    {
        self.x
    }

    pub fn y(&self) -> T
    where
        T: Copy,
    {
        self.y
    }

    /// Returns angle in radians from 0..2*PI
    pub(crate) fn angle(&self, c: Pt<T>) -> f64
    where
        T: Into<f64> + Copy + std::ops::Sub<Output = T>,
    {
        let (x, y): (f64, f64) = ((self.x() - c.x()).into(), (self.y() - c.y()).into());
        let a = y.atan2(x);
        // a
        if y < 0.0 {
            a
        } else {
            std::f64::consts::PI + std::f64::consts::PI - a
        }
    }

    pub(crate) fn radius(&self, c: Pt<T>) -> f64
    where
        T: Into<f64> + Copy + std::ops::Sub<Output = T>,
    {
        let Pt { x, y } = *self - c;
        let (x, y): (f64, f64) = (x.into(), y.into());
        (x.powi(2) + y.powi(2)).sqrt()
    }

    pub(crate) fn polar(&self, c: Pt<T>) -> Polar<f64>
    where
        T: Into<f64> + Copy + std::ops::Sub<Output = T>,
    {
        Polar {
            radius: self.radius(c),
            angle: self.angle(c),
        }
    }

    pub(crate) fn octant(&self, c: Pt<T>) -> u8
    where
        T: Copy
            + std::ops::Sub<Output = T>
            + std::cmp::PartialOrd<T>
            + From<u8>
            + std::ops::Neg<Output = T>,
    {
        let Pt { x, y } = *self - c;

        let zero = 0.into();
        if x >= zero && y < zero {
            if x > -y {
                return 0;
            } else {
                return 1;
            }
        }
        if x < zero && y < zero {
            if x > y {
                return 2;
            } else {
                return 3;
            }
        }
        if x < zero && y >= zero {
            if -x > y {
                return 4;
            } else {
                return 5;
            }
        }
        if x > y {
            return 7;
        } else {
            return 6;
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
    pub(crate) fn calc_error(&self, radius: i32, center: (i32, i32)) -> i32 {
        ((self.x.round() - center.0 as f64 + 1.0).powi(2)
            + (self.y.round() - center.1 as f64 - 0.5).powi(2)
            - radius.pow(2) as f64)
            .round() as i32
    }
    pub(crate) fn i32(&self) -> Pt<i32> {
        Pt {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }
}

impl Pt<i32> {
    pub(super) fn iter_to_real(self, oct: u8, c: Pt<i32>) -> Pt<i32> {
        match oct {
            1 => (self.y + c.x(), -self.x + c.y()).into(),
            2 => (self.x + c.x(), -self.y + c.y()).into(),
            3 => (-self.x + c.x(), -self.y + c.y()).into(),
            4 => (-self.y + c.x(), -self.x + c.y()).into(),
            5 => (-self.y + c.x(), self.x + c.y()).into(),
            6 => (-self.x + c.x(), self.y + c.y()).into(),
            7 => (self.x + c.x(), self.y + c.y()).into(),
            8 => (self.y + c.x(), self.x + c.y()).into(),
            _ => (self.x + c.x(), self.y + c.y()).into(),
        }
    }
    pub(crate) fn f64(&self) -> Pt<f64> {
        Pt {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
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
