mod slopes;

use crate::lines::BresIter;
use crate::{Point, Pt};
use slopes::{Edge, LineIter, Slope};

#[derive(Clone, Debug)]
pub struct ThickLine {
    top: LineIter,
    bot: LineIter,
    x: i32,
    end_x: i32,
    start: Edge,
    end: Edge,
    steep: bool,
}

impl ThickLine {
    fn check<P>(a: P, b: P) -> (Pt<i32>, Pt<i32>, bool)
    where
        P: crate::Point<i32>,
    {
        let (mut a, mut b) = (a.pt(), b.pt());

        if a.x() > b.x() {
            std::mem::swap(&mut a, &mut b);
        }

        match (a.x() - b.x()).abs() < (a.y() - b.y()).abs() {
            true => (a.transpose(), b.transpose(), true),
            false => (a, b, false),
        }
    }

    /// Creates a new iterator over a line with a specified thickness.
    ///
    /// # Panic
    ///
    /// Panics if `thickness` is 0.
    ///
    pub fn new<P>(a: P, b: P, thickness: u8) -> Self
    where
        P: crate::Point<i32>,
    {
        if thickness == 0 {
            panic!("Thickness cannot be 0");
        }

        let (a, b, steep) = Self::check(a, b);
        let slope = Slope::new(a, b);
        let thickness = (thickness) as i32;

        let wb = thickness / 2;
        let wt = thickness - wb;

        let start_top = slope.rev_slope_offset(a, -wt);
        let start_bot = slope.rev_slope_offset(a, wb);
        let end_top = slope.rev_slope_offset(b, -wt);
        let end_bot = slope.rev_slope_offset(b, wb);

        Self {
            top: LineIter::new(BresIter::new(start_top, end_top)).expect("Empty iterator"),
            bot: LineIter::new(BresIter::new(start_bot, end_bot)).expect("Empty iterator"),
            x: start_top.x().min(start_bot.x()),
            end_x: end_top.x().max(end_bot.x()),
            start: Edge::new(start_top, start_bot, Slope::new(start_bot, start_top)),
            end: Edge::new(end_top, end_bot, Slope::new(end_bot, end_top)),
            steep,
        }
    }

    // when 'if let chaining' is stabilized consider rewriting this
    // and removing the allow for clippy::unnecessary_unwrap
    #[allow(clippy::unnecessary_unwrap)]
    fn step(&mut self) -> (Pt<i32>, Pt<i32>) {
        let x = self.x;
        self.x += 1;

        let t = self.top.step(x);
        let b = self.bot.step(x);

        let top = if t.is_some() && x <= self.end.top.x() {
            t.unwrap()
        } else if x > self.end.top.x() {
            self.end.slope.pt_y(x) // needs end
        } else {
            self.start.slope.pt_y(x) // need start
        };

        let bot = if b.is_some() && x <= self.end.bot.x() {
            b.unwrap()
        } else if x > self.end.bot.x() {
            self.end.slope.pt_y(x) // needs end
        } else {
            self.start.slope.pt_y(x) // need start
        };

        if !self.steep {
            (top, bot)
        } else {
            (top.transpose(), bot.transpose())
        }
    }

    /// Draws a line with the specified thickness.
    pub fn draw<I, P, T>(image: &mut I, a: P, b: P, width: u8, color: I::Pixel)
    where
        I: image::GenericImage,
        P: crate::Point<T>,
        T: Into<i32>,
    {
        if width == 0 {
            return;
        }

        let a = Pt::new(a.x().into(), a.y().into());
        let b = Pt::new(b.x().into(), b.y().into());

        let it = Self::new(a, b, width);
        let steep = it.steep;

        for (a, b) in it {
            if a.is_negative() && b.is_negative() {
                continue;
            }

            if !steep {
                crate::lines::vertical_line(image, a.min_u32(), b.y().max(0) as u32, color);
            } else {
                crate::lines::horizontal_line(image, a.min_u32(), b.x().max(0) as u32, color);
            }
        }
    }
}

impl Iterator for ThickLine {
    type Item = (Pt<i32>, Pt<i32>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.end_x {
            return None;
        }

        let (t, b) = self.step();

        match self.top.it.steep() {
            false => Some((t, b)),
            true => Some((t.transpose(), b.transpose())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PURPLE, YELLOW};

    fn safe_plot(image: &mut image::RgbaImage, pt: Pt<i32>, color: image::Rgba<u8>) {
        if pt.x() >= 0
            && pt.x() < image.width() as i32
            && pt.y() >= 0
            && pt.y() < image.height() as i32
        {
            image.put_pixel(pt.x() as u32, pt.y() as u32, color);
        }
    }

    #[test]
    fn thick_iterator2() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let mut image = crate::test::img::blank((400, 400));

        let start = (50, 50);
        let end = (50, 100);
        let width = 20;
        let color = image::Rgba([255, 0, 0, 255]);

        let it = ThickLine::new(start, end, width);
        let steep = it.steep;

        let dbg = it.clone();
        log::debug!("{:#?}", it);

        for (a, b) in it {
            if !steep {
                crate::lines::vertical_line(&mut image, a.u32(), b.y() as u32, color);
            } else {
                crate::lines::horizontal_line(&mut image, a.u32(), b.x() as u32, color);
            }
        }

        if log::log_enabled!(log::Level::Trace) {
            safe_plot(&mut image, dbg.start.top, YELLOW);
            safe_plot(&mut image, dbg.start.bot, PURPLE);
            safe_plot(&mut image, dbg.end.top, YELLOW);
            safe_plot(&mut image, dbg.end.bot, PURPLE);
        }

        image.save("images/thick_line.png")
    }
}
