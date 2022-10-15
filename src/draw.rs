//! A simple wrapper around a mutable image reference for convenience.
//!

use crate::{Point, Pt};
use image::GenericImage;

/// A simple wrapper around a mutable image reference.
pub struct Draw<'i, I>
where
    I: image::GenericImage,
{
    image: &'i mut I,
}

impl<'i, I> Draw<'i, I>
where
    I: GenericImage,
{
    pub fn new(image: &'i mut I) -> Self {
        Self { image }
    }

    pub fn line<P, T>(self, a: P, b: P, color: I::Pixel) -> Self
    where
        P: Point<T>,
        T: Into<i32>,
    {
        let a = Pt::new(a.x().into(), a.y().into());
        let b = Pt::new(b.x().into(), b.y().into());

        crate::lines::line(self.image, a, b, color);

        self
    }
}
