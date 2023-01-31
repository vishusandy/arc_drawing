//! Allows drawing functions to be called using method chaining.
//!
//! This is a simple wrapper around a mutable image reference for convenience.
//!

#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]

use crate::conics;
use crate::lines;
use crate::ops;
use crate::shapes;
use crate::{Angle, Point, Pt};
use image::{GenericImage, Rgba, RgbaImage};

/// Allows drawing functions to be called using method chaining.
///
/// This is a simple wrapper around a mutable image reference.
///
/// ```
/// # use image::{Rgba, RgbaImage};
/// let mut image = RgbaImage::new(400, 400);
/// let color = Rgba([255, 0, 0, 255]);
///
/// let draw = freehand::new(&mut image);
/// // Draw a rectangle using lines
/// draw.line((10, 10), (50, 10), color)
///     .line((50, 10), (50, 50), color)
///     .line((50, 50), (10, 50), color)
///     .line((10, 50), (10, 10), color);
/// ```
pub struct Draw<'i, I>
where
    I: image::GenericImage,
{
    image: &'i mut I,
}

/// Methods for working with [`GenericImage`]s
impl<'i, I> Draw<'i, I>
where
    I: GenericImage,
{
    /// Creates a new wrapper around a mutable image refernce.
    ///
    /// This allows drawing functions to be called using method chaining.
    ///
    /// ```
    /// # use image::{Rgba, RgbaImage};
    /// let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::Draw::new(&mut image);
    /// ```
    pub fn new(image: &'i mut I) -> Self {
        Self { image }
    }

    /// Draws a straight line.
    ///
    /// See: [`lines::line`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a line between the two points
    /// draw.line((10, 10), (120, 180), Rgba([255, 0, 0, 255]));
    /// ```
    pub fn line<P, T>(self, a: P, b: P, color: I::Pixel) -> Self
    where
        P: Point<T>,
        T: Into<i32> + Copy,
    {
        let a = Pt::new(a.x().into(), a.y().into());
        let b = Pt::new(b.x().into(), b.y().into());

        lines::line(self.image, a, b, color);
        self
    }

    /// Draws a dashed line between two points.
    ///
    /// See [`lines::dashed_line`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a 3px dashed line between the two points
    /// draw.dashed_line((10, 10), (120, 180), 3, Rgba([255, 0, 0, 255]));
    /// ```
    pub fn dashed_line<P, T>(self, a: P, b: P, dash_width: u16, color: I::Pixel) -> Self
    where
        P: Point<T>,
        T: Into<i32> + Copy,
    {
        let a = Pt::new(a.x().into(), a.y().into());
        let b = Pt::new(b.x().into(), b.y().into());

        lines::dashed_line(self.image, a, b, dash_width, color);
        self
    }

    /// Draws a line from each point to the next.
    ///
    /// Does not connect the start and end points.
    ///
    /// See [`lines::path`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a line between each of the points
    /// let points = [(10, 10), (120, 180)];
    /// draw.path(points, Rgba([255, 0, 0, 255]));
    /// ```
    pub fn path<P, It>(self, points: It, color: I::Pixel) -> Self
    where
        P: Point<i32>,
        It: IntoIterator<Item = P>,
    {
        lines::path(self.image, points, color);
        self
    }

    /// Draws a rectangle.
    ///
    /// See [`shapes::rectangle`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// draw.rectangle((10, 10), 50, 50, Rgba([255, 0, 0, 255]));
    /// ```
    pub fn rectangle<P>(self, pt: P, height: u32, width: u32, color: I::Pixel) -> Self
    where
        P: Point<u32>,
    {
        shapes::rectangle(self.image, pt, height, width, color);
        self
    }

    /// Draws a filled rectangle
    ///
    /// See [`shapes::rectangle_filled`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// draw.rectangle_filled((10, 10), 50, 50, Rgba([255, 0, 0, 255]));
    /// ```
    pub fn rectangle_filled<P>(self, pt: P, height: u32, width: u32, color: I::Pixel) -> Self
    where
        P: Point<u32>,
    {
        shapes::rectangle_filled(self.image, pt, height, width, color);
        self
    }

    /// Draws a circular arc.
    ///
    /// See [`conics::arc`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red arc from 0° to 55°, with a radius of 180 pixels from the image center.
    /// draw.arc(0, 55, 180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    pub fn arc<A, C, T>(
        self,
        start_angle: A,
        end_angle: A,
        radius: T,
        center: C,
        color: I::Pixel,
    ) -> Self
    where
        A: Angle,
        C: Point<T>,
        T: Into<i32> + Copy,
    {
        conics::arc(self.image, start_angle, end_angle, radius, center, color);
        self
    }

    /// Draws a circle.
    ///
    /// See [`conics::circle`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red circle with a radius of 180 pixels from the image center.
    /// draw.circle(180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    pub fn circle<C, T>(self, radius: T, center: C, color: I::Pixel) -> Self
    where
        C: Point<T>,
        T: Into<i32> + Copy,
    {
        conics::circle(self.image, radius, center, color);
        self
    }

    /// Draws a filled pie slice.
    ///
    /// See [`conics::pie_slice_filled`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a pie slice from 0° to 55°, with a radius of 180 pixels from the image center.
    /// draw.pie_slice_filled(0, 55, 180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    pub fn pie_slice_filled<A, C>(
        self,
        start_angle: A,
        end_angle: A,
        radius: i32,
        center: C,
        color: I::Pixel,
    ) -> Self
    where
        A: Angle,
        C: Point<i32>,
        I: GenericImage,
    {
        conics::pie_slice_filled(self.image, start_angle, end_angle, radius, center, color);
        self
    }

    /// Draws a thick arc.
    ///
    /// See [`conics::thick_arc`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws an arc, with a thickness of 3, from 0° to 55°, with a radius of 180 pixels from the image center.
    /// draw.thick_arc(0, 55, 180, 3, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    pub fn thick_arc<A, C>(
        self,
        start_angle: A,
        end_angle: A,
        radius: i32,
        thickness: i16,
        center: C,
        color: I::Pixel,
    ) -> Self
    where
        A: Angle,
        C: Point<i32>,
    {
        conics::thick_arc(
            self.image,
            start_angle,
            end_angle,
            radius,
            thickness,
            center,
            color,
        );
        self
    }

    /// Draws a thick circle.
    ///
    /// See [`conics::thick_circle`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a circle with a thickness of 3 and a radius of 180 pixels from the image center.
    /// draw.thick_circle(180, 3, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    pub fn thick_circle<C>(self, radius: i32, thickness: i16, center: C, color: I::Pixel) -> Self
    where
        C: Point<i32>,
    {
        conics::thick_circle(self.image, radius, thickness, center, color);
        self
    }

    /// Draws an annulus (a filled donut)
    ///
    /// See: [`conics::annulus`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws an annulus from 0° to 55°, with an inner radius of 120 and outer radius of 180 pixels from the image center.
    /// draw.annulus(0, 55, 120, 180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    pub fn annulus<A, C>(
        self,
        start_angle: A,
        end_angle: A,
        inner_radius: i32,
        outer_radius: i32,
        center: C,
        color: I::Pixel,
    ) -> Self
    where
        A: Angle,
        C: Point<i32>,
    {
        conics::annulus(
            self.image,
            start_angle,
            end_angle,
            inner_radius,
            outer_radius,
            center,
            color,
        );
        self
    }
}

/// Methods for working with [`RgbaImage`]s.
impl<'i> Draw<'i, RgbaImage> {
    /// Draws an antialiased arc.
    ///
    /// See [`conics::antialiased_arc`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // draws an anti-aliased arc from 0° to 55° with a radius of 180 pixels from the image center.
    /// draw.antialiased_arc(0, 55, 180, (200, 200), Rgba([255, 0, 0, 255]));
    /// ```
    pub fn antialiased_arc<A, C, T>(
        self,
        start_angle: A,
        end_angle: A,
        radius: T,
        center: C,
        color: Rgba<u8>,
    ) -> Self
    where
        A: Angle,
        C: Point<T>,
        T: Into<f64> + Copy,
    {
        conics::antialiased_arc(self.image, start_angle, end_angle, radius, center, color);
        self
    }

    /// Draws a dashed line with a specified opacity.
    ///
    /// See [`lines::dashed_line_alpha`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red line with a 3px dash and 50% opacity.
    /// draw.dashed_line_alpha((0, 10), (200, 200), 5u8, 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    pub fn dashed_line_alpha<P, W>(
        self,
        a: P,
        b: P,
        dash_width: W,
        opacity: f32,
        color: Rgba<u8>,
    ) -> Self
    where
        P: Point<i32>,
        W: Into<u16>,
    {
        lines::dashed_line_alpha(self.image, a, b, dash_width, opacity, color);
        self
    }

    /// Draws a line with a specified opacity.
    ///
    /// See [`lines::line_alpha`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red line with 50% opacity.
    /// draw.line_alpha((0, 10), (200, 200), 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    pub fn line_alpha<P>(self, a: P, b: P, opacity: f32, color: Rgba<u8>) -> Self
    where
        P: Point<i32>,
    {
        lines::line_alpha(self.image, a, b, opacity, color);
        self
    }

    /// Draws a thick anti-aliased line.
    ///
    /// See: [`lines::thick_line`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red anti-aliased line with a width of 1.5
    /// draw.thick_line((0, 10), (200, 200), 1.5, Rgba([255, 0, 0, 255]));
    /// ```
    pub fn thick_line<P, T>(self, a: P, b: P, width: f32, color: Rgba<u8>) -> Self
    where
        P: Point<T>,
        T: Into<i32> + Copy,
    {
        lines::thick_line(self.image, a, b, width, color);
        self
    }

    /// Draws a rectangle with the specified opacity.
    ///
    /// See [`shapes::rectangle_alpha`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a red rectangle with 50% opacity.
    /// draw.rectangle_alpha((0, 10), 50, 50, 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    pub fn rectangle_alpha<P>(
        self,
        pt: P,
        height: u32,
        width: u32,
        opacity: f32,
        color: Rgba<u8>,
    ) -> Self
    where
        P: Point<u32>,
    {
        shapes::rectangle_alpha(self.image, pt, height, width, opacity, color);
        self
    }

    /// Draws a filled rectangle with the specified opacity.
    ///
    /// See [`shapes::rectangle_filled_alpha`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Draws a filled red rectangle with 50% opacity.
    /// draw.rectangle_filled_alpha((0, 10), 50, 50, 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    pub fn rectangle_filled_alpha<P>(
        self,
        pt: P,
        height: u32,
        width: u32,
        opacity: f32,
        color: Rgba<u8>,
    ) -> Self
    where
        P: Point<u32>,
    {
        shapes::rectangle_filled_alpha(self.image, pt, height, width, opacity, color);
        self
    }

    /// Blends a color into an image.
    ///
    /// The resulting color's alpha channel will ignore the specified color's alpha
    /// value and use `opacity` to blend the colors together.  The specified
    /// color's alpha value will only be used for the final alpha channel value.
    ///
    /// See [`ops::blend_at`]
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Blends a red pixel into the image at (0, 10) with 50% opacity
    /// draw.blend_at(0, 10, 0.5, Rgba([255, 0, 0, 255]));
    pub fn blend_at(self, x: u32, y: u32, opacity: f32, color: Rgba<u8>) -> Self {
        ops::blend_at(self.image, x, y, opacity, color);
        self
    }

    /// Blend a specified color into an existing image coordinate.  This ignores `color`'s
    /// alpha value and instead uses `opacity` which is a floating point number from 0.0 to 1.0.
    ///
    /// The resulting color's alpha channel will ignore the specified color's alpha
    /// value and use `opacity` to blend the colors together.  The specified
    /// color's alpha value will only be used for the final alpha channel value.
    ///
    /// A few safety checks are skipped here for performance.
    ///
    /// # Safety
    /// The x and y coordinates must be less than the image width and height, respectively.
    ///
    /// Also, `opacity` should be in the range `(0..=1.0)`.
    ///
    /// ```
    /// # use image::{RgbaImage, Rgba};
    /// # let mut image = RgbaImage::new(400, 400);
    ///
    /// let draw = freehand::new(&mut image);
    /// // Blends a red pixel into the image at (0, 10) with 50% opacity
    /// draw.blend_at(0, 10, 0.5, Rgba([255, 0, 0, 255]));
    /// ```
    pub unsafe fn blend_at_unchecked(self, x: u32, y: u32, opacity: f32, color: Rgba<u8>) -> Self {
        ops::blend_at_unchecked(self.image, x, y, opacity, color);
        self
    }
}

/// Creates a new wrapper around a mutable image.
///
/// This allows drawing functions to be called using method chaining.
///
/// ```
/// # use image::{Rgba, RgbaImage};
/// let mut image = RgbaImage::new(400, 400);
///
/// let draw = freehand::new(&mut image);
/// ```
pub fn new<I>(image: &mut I) -> Draw<I>
where
    I: image::GenericImage,
{
    Draw { image }
}
