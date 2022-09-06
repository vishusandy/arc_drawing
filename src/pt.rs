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
