use crate::pt::{Point, Pt};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
/// An iterator between two points on a line.  Uses Bresenham's algorithm.
///
/// # Example
///
/// ```
/// use freehand::lines::LineIter;
///
/// for freehand::Pt {x, y} in LineIter::new((0, 0), (399, 399)) {
///     // ...
/// }
/// ```
// https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm#timings-fifth-and-final-attempt
pub struct LineIter {
    /// Current position
    pt: Pt<i32>,
    /// Where to end
    end: Pt<i32>,
    d: i32,
    /// Amount added to decision parameter every step
    dy: i32,
    /// Amount subtracted from decision parameter on y steps
    dx: i32,
    /// Amount added to y on y steps
    y_step: i32,
    /// If steep the x, y coordinates are transposed
    steep: bool,
}

impl LineIter {
    /// Creates an iterator between two points on a line.
    ///
    ///
    ///
    /// ```
    /// use freehand::lines::LineIter;
    ///
    /// for freehand::Pt {x, y} in LineIter::new((0, 0), (399, 399)) {
    ///     // ...
    /// }
    /// ```
    pub fn new<P>(a: P, b: P) -> Self
    where
        P: Point<i32>,
    {
        let (mut a, mut b) = (a.pt(), b.pt());

        let steep = (a.x() - b.x()).abs() < (a.y() - b.y()).abs();
        if steep {
            a.swap();
            b.swap();
        }

        if a.x() > b.x() {
            std::mem::swap(&mut a, &mut b);
        }

        let d = 0;

        // match seems more succinct than if/elseif/else
        #[allow(clippy::match_bool)]
        let y_step = match a.y() > b.y() {
            true => -1,
            false if a.y() == b.y() => 0,
            false => 1,
        };

        Self {
            pt: a,
            end: b,
            d,
            dy: (b.y() - a.y()).abs() * 2,
            dx: b.x() - a.x(),
            y_step,
            steep,
        }
    }

    /// Returns the current position in the line.
    #[must_use]
    pub fn pt(&self) -> Pt<i32> {
        if self.steep {
            self.pt.transpose()
        } else {
            self.pt
        }
    }

    /// Returns the end point of the line.
    #[must_use]
    pub fn end(&self) -> Pt<i32> {
        if self.steep {
            self.end.transpose()
        } else {
            self.end
        }
    }

    /// Returns the decision parameter that decides whether to change y or not.
    ///
    /// If `steep` is `true` this represents whether to change x.
    #[must_use]
    pub fn d(&self) -> i32 {
        self.d
    }

    /// Error amount added to the decision parameter every step.
    ///
    /// Does not account for `steep == true` (which switches x and y)
    #[must_use]
    pub fn dy(&self) -> i32 {
        self.dy
    }

    /// Error amount subtracted from the decision parameter when y changes.
    ///
    /// Does not account for `steep == true` (which switches x and y)
    #[must_use]
    pub fn dx(&self) -> i32 {
        self.dx
    }

    /// Amount added to y on y steps.
    ///
    /// Does not account for `steep == true` (which switches x and y)
    #[must_use]
    pub fn y_step(&self) -> i32 {
        self.y_step
    }

    /// If steep is true the x, y coordinates will be transposed.
    #[must_use]
    pub fn steep(&self) -> bool {
        self.steep
    }
}

impl Iterator for LineIter {
    type Item = Pt<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pt = self.pt;
        if self.pt.x() > self.end.x() {
            return None;
        }

        self.pt.add_x(1);
        self.d += self.dy;

        if self.d > self.dx {
            self.pt.add_y(self.y_step);
            self.d -= self.dx * 2;
        }

        if self.steep {
            pt.swap();
        }

        Some(pt)
    }
}
