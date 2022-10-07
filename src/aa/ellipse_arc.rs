#![allow(dead_code)]
use crate::Pt;

/*
https://www.includehelp.com/computer-graphics/mid-point-ellipse-algorithm.aspx
https://www.javatpoint.com/computer-graphics-midpoint-ellipse-algorithm
https://www.geeksforgeeks.org/midpoint-ellipse-drawing-algorithm/

https://math.stackexchange.com/questions/2818140/how-to-calculate-coordinates-of-point-p-on-an-ellipse-that-has-a-tangent-line-at
https://math.stackexchange.com/questions/739570/intersection-of-a-45-degree-angle-and-an-ellipse
https://math.stackexchange.com/questions/22064/calculating-a-point-that-lies-on-an-ellipse-given-an-angle

https://math.stackexchange.com/questions/3638086/calculate-coordinates-of-a-circle

https://en.wikipedia.org/wiki/Ellipse

*/

#[derive(Clone, Debug)]
struct Ellipse {
    x: f64,
    y: f64,
    /// horizontal radius
    xr: f64,
    /// veritcal radius
    yr: f64,
    /// center
    c: Pt<f64>,
    /// End X coordinate
    ex: f64,
}
impl Ellipse {
    fn blank_ellipse(xr: f64, yr: f64, c: Pt<f64>) -> Self {
        Self {
            x: 0.0,
            y: xr,
            xr,
            yr,
            c,
            ex: Self::ffd(xr, yr),
        }
    }

    fn end(&self) -> bool {
        if self.x > self.xr {
            return true;
        }
        false
    }

    fn next_octant(&mut self) -> bool {
        false
    }

    /// Calculate the 45 degree point assuming x is fast direction (solving for y)
    fn point(angle: f64, xr: f64, yr: f64) -> Pt<f64> {
        // https://math.stackexchange.com/questions/22064/calculating-a-point-that-lies-on-an-ellipse-given-an-angle
        let x = (xr * yr) / (yr.powi(2) + xr.powi(2) * angle.tan().powi(2)).sqrt();
        let y = (xr * yr) / (xr.powi(2) + (yr.powi(2) / angle.tan().powi(2))).sqrt();
        Pt::new(x, y)
    }

    fn ffd(xr: f64, yr: f64) -> f64 {
        (xr * yr) / (yr.powi(2) + xr.powi(2) * (std::f64::consts::PI * 0.125).tan().powi(2)).sqrt()
    }

    fn step(&mut self) -> Option<Pt<f64>> {
        if self.end() {
            return None;
        }

        let y = (self.yr * self.yr * (1.0 - (self.x * self.x / (self.xr * self.xr))))
            .sqrt()
            .round();

        let ret = Some(Pt::new(self.x, y));
        self.x += 1.0;
        ret
    }
    fn draw(&mut self, image: &mut image::RgbaImage, color: image::Rgba<u8>) {
        let c = self.c;
        for Pt { x, y } in self {
            image.put_pixel((x + c.x()) as u32, (y + c.y()) as u32, color);
        }
    }
}

impl Iterator for Ellipse {
    type Item = Pt<f64>;
    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ellipse() -> Result<(), image::ImageError> {
        crate::logger(crate::LOG_LEVEL);
        let xr = 70.0;
        let yr = 50.0;
        let c = Pt::new(200.0, 200.0);
        let color = image::Rgba([255, 0, 0, 255]);
        let mut image =
            // crate::test::img::setup_ellipse(h as i32, v as i32, (c.x() as i32, c.y() as i32));
            crate::test::img::guidelines();
        // let mut image = crate::test::img::blank(Pt::new(crate::IMG_SIZE, crate::IMG_SIZE));
        let mut e = Ellipse::blank_ellipse(xr, yr, c);

        e.draw(&mut image, color);

        let angle = std::f64::consts::PI * 0.125;
        let pt = Ellipse::point(angle, xr, yr);
        image.put_pixel(
            (pt.x().round() + c.x()) as u32,
            (pt.y().round() + c.y()) as u32,
            image::Rgba([0, 0, 255, 255]),
        );

        image.save("images/ellipse.png")
    }
}
