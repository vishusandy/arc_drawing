//! Simple point manipulations.
//!
//! # [`Point`] trait
//! The [`Point`] trait helps make functions easier to use.  Functions can take a
//! generic argument implementing [`Point`] instead of a function that takes an
//! explicit [`Pt`].  This will allow the functions to be called with an `(x, y)`
//! tuple as well.
//!
//! # [`Pt`] struct
//!
//! The [`Pt`] struct represents an x, y coordinate while also providing some
//! basic manipulation.
//!
//!
//!

/// Represents x, y coordinates for a type.  Allows functions to be generic over
/// types that represent x, y coordinates.
pub trait Point<T>
where
    T: Copy,
    Self: Copy,
{
    /// Return a [`Pt`]
    fn pt(&self) -> Pt<T> {
        Pt::new(self.x(), self.y())
    }

    /// Return an `(x, y)` tuple
    fn tuple(&self) -> (T, T) {
        (self.x(), self.y())
    }

    /// Return the x coordinate
    fn x(&self) -> T;

    /// Return the y coordinate
    fn y(&self) -> T;

    /// Replace the x coordinate
    fn set_x(&mut self, x: T);

    /// Replace the y coordinate
    fn set_y(&mut self, y: T);

    /// Creates a new [`Pt`] by switching the x and y coordinates
    fn transpose(&self) -> Pt<T> {
        Pt::new(self.y(), self.x())
    }

    /// Creates a new [`Pt`] by adding a value to x
    fn plus_x(&self, x: T) -> Pt<T>
    where
        T: std::ops::Add<Output = T> + Copy,
    {
        Pt::new(self.x() + x, self.y())
    }

    /// Creates a new [`Pt`] by adding a value to y
    fn plus_y(&self, y: T) -> Pt<T>
    where
        T: std::ops::Add<Output = T> + Copy,
    {
        Pt::new(self.x(), self.y() + y)
    }
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

    fn set_x(&mut self, x: T) {
        self.0 = x;
    }

    fn set_y(&mut self, y: T) {
        self.1 = y;
    }
}

impl<T> Point<T> for Pt<T>
where
    T: Copy,
{
    fn pt(&self) -> Self {
        *self
    }

    fn x(&self) -> T {
        self.x()
    }

    fn y(&self) -> T {
        self.y()
    }

    fn set_x(&mut self, x: T) {
        self.x = x;
    }

    fn set_y(&mut self, y: T) {
        self.y = y;
    }
}

/// Represents an x, y point and provides basic manipulation.
///
/// This is mostly intended for use within the crate, however it is provided as
/// public in order for convenience when working with this crate.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Pt<T> {
    /// The x coordinate
    pub x: T,
    /// The y coordinate
    pub y: T,
}

impl<T> Pt<T> {
    /// Create a new `Pt` from x, y coordinates
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    /// Return the x coordinate
    pub const fn x(&self) -> T
    where
        T: Copy,
    {
        self.x
    }

    #[inline]
    /// Return the y coordinate
    pub const fn y(&self) -> T
    where
        T: Copy,
    {
        self.y
    }

    /// Swap the x and y values
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }

    /// Add to the x value
    pub fn add_x(&mut self, x: T)
    where
        T: std::ops::AddAssign,
    {
        self.x += x;
    }

    /// Add to the y value
    pub fn add_y(&mut self, y: T)
    where
        T: std::ops::AddAssign,
    {
        self.y += y;
    }

    /// Subtract a value from the x value
    pub fn sub_x(&mut self, x: T)
    where
        T: std::ops::SubAssign,
    {
        self.x -= x;
    }

    /// Subtract a value from the y value
    pub fn sub_y(&mut self, y: T)
    where
        T: std::ops::SubAssign,
    {
        self.y -= y;
    }

    /// Multiply the x value
    pub fn mul_x(&mut self, x: T)
    where
        T: std::ops::MulAssign,
    {
        self.x *= x;
    }

    /// Multiply the y value
    pub fn mul_y(&mut self, y: T)
    where
        T: std::ops::MulAssign,
    {
        self.y *= y;
    }

    /// Divide the x value
    pub fn div_x(&mut self, x: T)
    where
        T: std::ops::DivAssign,
    {
        self.x /= x;
    }

    /// Divide the y value
    pub fn div_y(&mut self, y: T)
    where
        T: std::ops::DivAssign,
    {
        self.y /= y;
    }

    /// Add a number to the x and y coordinates
    #[must_use]
    pub fn add(&self, value: T) -> Self
    where
        T: Copy + std::ops::Add<Output = T>,
    {
        Self {
            x: self.x + value,
            y: self.y + value,
        }
    }

    /// Subtract a number from the x and y coordinates
    #[must_use]
    pub fn sub(&self, value: T) -> Self
    where
        T: Copy + std::ops::Sub<Output = T>,
    {
        Self {
            x: self.x - value,
            y: self.y - value,
        }
    }

    /// Multiply a number with the x and y coordinates
    #[must_use]
    pub fn mul(&self, value: T) -> Self
    where
        T: Copy + std::ops::Mul<Output = T>,
    {
        Self {
            x: self.x * value,
            y: self.y * value,
        }
    }

    /// Divide both of the x and y coordinates by a number
    #[must_use]
    pub fn div(&self, value: T) -> Self
    where
        T: Copy + std::ops::Div<Output = T>,
    {
        Self {
            x: self.x / value,
            y: self.y / value,
        }
    }

    /// Convert real image coordinates to those used by an iterator in octant 7.
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
            // 7 => Pt::new(self.x, self.y),
            8 => Pt::new(self.y, self.x),
            _ => Pt::new(self.x, self.y),
        }
    }

    /// Convert iterator coordinates in quadrant 4 (octants 7 & 8) to those used by an image.
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

    /// Convert real image coordinates to those used in an iterator in quadrant 4 (octants 7 & 8)
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
    /// Calculates a point on a circle using the given angle, radius, and circle center.
    pub fn from_radian<T, P>(angle: f64, radius: T, center: P) -> Self
    where
        T: Into<f64> + Copy,
        P: crate::pt::Point<T>,
    {
        let x = center.x().into() + radius.into() * angle.cos();
        let y = center.y().into() - radius.into() * angle.sin();

        Self { x, y }
    }

    /// Calculates a point on a circle using an angle, radius, and circle center.
    ///
    /// Floating-point numbers will be treated as radians while other numbers will be
    /// treated as degrees.
    pub fn from_angle<A, P, T>(angle: A, radius: T, center: P) -> Self
    where
        A: crate::angle::Angle,
        P: crate::pt::Point<T>,
        T: Into<f64> + Copy,
    {
        let x = center.x().into() + radius.into() * angle.radians().cos();
        let y = center.y().into() - radius.into() * angle.radians().sin();

        Self { x, y }
    }

    /// Round and cast to a `Pt<i32>`.
    #[must_use]
    pub fn i32(&self) -> Pt<i32> {
        Pt {
            x: self.x.round() as i32,
            y: self.y.round() as i32,
        }
    }

    /// Casts to a `Pt<u32>` with `abs()` and `round()`
    #[must_use]
    pub fn u32(&self) -> Pt<u32> {
        Pt {
            x: self.x.abs().round() as u32,
            y: self.y.abs().round() as u32,
        }
    }
}

impl Pt<i32> {
    /// Casts to a `Pt<u32>`
    #[must_use]
    pub const fn u32(&self) -> Pt<u32> {
        Pt {
            x: self.x as u32,
            y: self.y as u32,
        }
    }

    /// A safer conversion to a `Pt<u32>` using `unsigned_abs()`
    #[must_use]
    pub const fn abs_u32(&self) -> Pt<u32> {
        Pt {
            x: self.x.unsigned_abs(),
            y: self.y.unsigned_abs(),
        }
    }

    /// Converts an i32 to u32 by changing negatives to 0
    #[must_use]
    pub fn min_u32(&self) -> Pt<u32> {
        Pt {
            x: self.x.max(0) as u32,
            y: self.y.max(0) as u32,
        }
    }

    /// Returns whether both coordinates are negative
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.x.is_negative() | self.y.is_negative()
    }
}

impl Pt<u32> {
    #[must_use]
    // safe because of the assert!()
    #[allow(clippy::cast_possible_wrap)]
    /// Cast to a `Pt<i32>`
    ///
    /// # Panics
    ///
    /// Panics if the values cannot fit into an i32
    pub const fn i32(&self) -> Pt<i32> {
        assert!(self.x <= std::i32::MAX as u32 && self.y <= std::i32::MAX as u32);
        Pt {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    /// Cast to a `Pt<f32>`
    #[must_use]
    pub const fn f32(&self) -> Pt<f32> {
        Pt {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    /// Cast to a `Pt<f64>`
    #[must_use]
    pub const fn f64(&self) -> Pt<f64> {
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

impl TryFrom<Pt<u32>> for Pt<i32> {
    type Error = &'static str;

    fn try_from(pt: Pt<u32>) -> Result<Self, Self::Error> {
        if pt.x <= std::i32::MAX as u32 && pt.y <= std::i32::MAX as u32 {
            // safe because of the if check
            #[allow(clippy::cast_possible_wrap)]
            Ok(Self {
                x: pt.x as i32,
                y: pt.y as i32,
            })
        } else {
            Err("bounds exceeds i32::MAX")
        }
    }
}

// impl From<Pt<u32>> for Pt<i32> {
//     #[allow(clippy::cast_possible_wrap)]
//     fn from(pt: Pt<u32>) -> Self {
//         Self {
//             x: pt.x as i32,
//             y: pt.y as i32,
//         }
//     }
// }

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

impl std::convert::TryFrom<Pt<i32>> for Pt<u32> {
    type Error = &'static str;

    fn try_from(pt: Pt<i32>) -> Result<Self, Self::Error> {
        if pt.x().is_negative() || pt.y().is_negative() {
            Err("Negative i32 values cannot be converted to u32")
        } else {
            Ok(Pt::new(pt.x() as u32, pt.y() as u32))
        }
    }
}

// impl std::convert::TryFrom<Pt<u32>> for Pt<i32> {
//     type Error = &'static str;

//     fn try_from(pt: Pt<u32>) -> Result<Self, Self::Error> {
//         if pt.x() > std::i32::MAX as u32 || pt.y() > std::i32::MAX as u32 {
//             Err("")
//         } else {
//             Ok(Pt::new(pt.x() as u32, pt.y() as u32))
//         }
//     }
// }
