use crate::pt::Pt;

const RAD: f64 = std::f64::consts::PI / 4.0;

fn angle_octant(angle: f64) -> u8 {
    if angle < RAD {
        return 1;
    }
    if angle < RAD * 2.0 {
        return 2;
    }
    if angle < RAD * 3.0 {
        return 3;
    }
    if angle < RAD * 4.0 {
        return 4;
    }
    if angle < RAD * 5.0 {
        return 5;
    }
    if angle < RAD * 6.0 {
        return 6;
    }
    if angle < RAD * 7.0 {
        return 7;
    }
    if angle < RAD * 8.0 {
        return 8;
    }
    angle_octant(angle % (RAD * 8.0))
}

/// Converts coordinates from the iterator to actual coordinates
pub(super) mod bres_to {
    use std::ops::Neg;
    pub(in super::super) fn o1<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.1, -c.0)
    }
    pub(in super::super) fn o2<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.0, -c.1)
    }
    pub(in super::super) fn o3<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.0, -c.1)
    }
    pub(in super::super) fn o4<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.1, -c.0)
    }
    pub(in super::super) fn o5<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.1, c.0)
    }
    pub(in super::super) fn o6<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.0, c.1)
    }
    pub(in super::super) fn o7<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        c
    }
    pub(in super::super) fn o8<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.1, c.0)
    }
}
// Converts actual coordinates to iterator coordinates
mod bres_from {
    use std::ops::Neg;
    pub(in super::super) fn o1<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.1, c.0) // negative sign needed to be swapped from y to x
    }
    pub(in super::super) fn o2<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.0, -c.1)
    }
    pub(in super::super) fn o3<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.0, -c.1)
    }
    pub(in super::super) fn o4<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.1, -c.0)
    }
    pub(in super::super) fn o5<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.1, -c.0) // needed negative y instead of positive
    }
    pub(in super::super) fn o6<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.0, c.1)
    }
    pub(in super::super) fn o7<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        c
    }
    pub(in super::super) fn o8<T: Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.1, c.0)
    }
}

#[derive(Clone, Debug)]
struct OctRev {
    x: i32,
    y: i32,
    d: i32,
    c: Pt<i32>,
}
impl OctRev {
    fn new(r: i32, c: Pt<i32>) -> Self {
        let start = std::f64::consts::PI / 4.0 * 7.0;
        let pt = Pt::from_radian(start, r, c.into()).real_to_iter(8, c.into());
        let d: i32 = ((pt.x().round() as f64 + 1.0).powi(2) + (pt.y().round() as f64 - 0.5).powi(2)
            - r.pow(2) as f64)
            .round() as i32;
        let Pt { x, y } = pt.i32();
        println!(
            "Start: start={:.4} x={} y={} d={} c={:?}",
            start, x, y, d, c
        );
        Self {
            x: x - 1,
            y: y - 0,
            d,
            c,
        }
    }
}
impl Iterator for OctRev {
    type Item = (i32, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.y == 0 || self.x == 0 || self.x >= self.c.x() - 10 {
            return None;
        }
        let (x, y) = (self.x, self.y);
        println!(" d={}\tx={}\ty={}", self.d, x, y);
        println!("\t\tx+c={}\ty+c={}", x + self.c.x(), y + self.c.y());
        self.y -= 1;
        if self.d > 0 {
            self.x += 1;
            self.d += 2 * (self.y - self.x) - 1;
        } else {
            self.d += 2 * self.y + 1;
        }
        Some((x + self.c.x(), y + self.c.y()))
    }
}

/// First define the name of the struct, pass an identifier for x and y, then pass how to translate and reverse translate the coordinates.
macro_rules! bres_oct {
    ( $o:ident, $oct:literal, $x:ident, $y:ident, ( $ex:expr, $ey:expr ), ( $rx:expr, $ry:expr )) => {
        #[derive(Clone, Debug)]
        pub struct $o {
            x: i32,
            y: i32,
            d: i32,
            e: Option<std::num::NonZeroI32>,
            c: (i32, i32),
        }
        impl $o {
            const OCT: u8 = $oct;

            /// Iterator over a a partial or whole octant (depending on parameters)
            pub fn arc(
                start: Option<f64>,
                end: Option<f64>,
                radius: i32,
                center: (i32, i32),
            ) -> Self {
                match (start, end) {
                    (Some(s), Some(e)) => Self::segment(s, e, radius, center),
                    (None, Some(e)) => Self::until(e, radius, center),
                    (Some(s), None) => Self::at(s, radius, center),
                    _ => Self::full(radius, center),
                }
            }

            /// Iterator over the entire octet
            pub fn full(radius: i32, center: (i32, i32)) -> Self {
                Self {
                    x: 0,
                    y: radius,
                    d: 1 - radius,
                    e: None,
                    c: center,
                }
            }

            /// Iterator over partial octant
            pub fn segment(start: f64, end: f64, radius: i32, center: (i32, i32)) -> Self {
                let c = center;
                let mut st = start % RAD + RAD * 6.0;
                let mut et = end % RAD + RAD * 6.0;
                if Self::OCT % 2 == 0 {
                    std::mem::swap(&mut st, &mut et);
                    println!("original end={:.4}", et);
                    et = RAD * 7.0 - et + RAD * 6.0; // remember the iterator is in octet 7
                }
                println!("start={:.4} end={:.4}", st, et);
                let Pt { x: x1, y: y1 } = Pt::from_radian(st, radius, c);
                let Pt { x: x2, y: y2 } = Pt::from_radian(et, radius, c);
                let (x, y) = (x1.round() as i32 - c.0, y1.round() as i32 - c.1);
                let (x2, y2) = (x2.round() as i32, y2.round() as i32);
                let d: i32 = ((x1.round() - c.0 as f64 + 1.0).powi(2)
                    + (y1.round() - c.1 as f64 - 0.5).powi(2)
                    - radius.pow(2) as f64)
                    .round() as i32;
                let e = if x2 == 0 {
                    None
                } else {
                    unsafe { Some(std::num::NonZeroI32::new_unchecked(x2.abs())) }
                };
                Self { x, y, d, e, c }
            }

            /// Iterator starting at a specified angle and going through the rest of the octant
            pub fn at(start_theta: f64, radius: i32, center: (i32, i32)) -> Self {
                if Self::OCT % 2 == 0 {
                    Self::until_local(start_theta, radius, center)
                } else {
                    Self::at_local(start_theta, radius, center)
                }
            }

            /// Iterator starting at the beginning of the octant and stopping at a specified angle
            pub fn until(end_theta: f64, radius: i32, center: (i32, i32)) -> Self {
                if Self::OCT % 2 == 0 {
                    Self::at_local(end_theta, radius, center)
                } else {
                    Self::until_local(end_theta, radius, center)
                }
            }

            fn at_local(start_theta: f64, radius: i32, center: (i32, i32)) -> Self {
                let theta = start_theta % RAD + RAD * 6.0;
                let Pt { x: x1, y: y1 } = Pt::from_radian(theta, radius, center);
                let d: i32 = ((x1.round() - center.0 as f64 + 1.0).powi(2)
                    + (y1.round() - center.1 as f64 - 0.5).powi(2)
                    - radius.pow(2) as f64)
                    .round() as i32;
                let (x, y) = (x1.round() as i32 - center.0, y1.round() as i32 - center.1);
                Self {
                    x,
                    y,
                    d,
                    e: None,
                    c: center,
                }
            }

            fn until_local(end_theta: f64, radius: i32, center: (i32, i32)) -> Self {
                let theta = end_theta % RAD + RAD * 6.0;
                let pt = Pt::from_radian(theta, radius, center);
                let ox = pt.x().round() as i32;
                let e = if ox == 0 {
                    None
                } else {
                    // This is safe becuase we already check if x is 0
                    unsafe { Some(std::num::NonZeroI32::new_unchecked(ox.abs())) }
                };
                Self {
                    x: 0,
                    y: radius,
                    d: 1 - radius,
                    e,
                    c: center,
                }
            }

            /// Return the iterator's center position
            pub fn center(&self) -> (i32, i32) {
                self.c
            }

            /// Convert coordinates from iterator coords to final coordinates
            pub fn translate(pt: Pt<i32>) -> Pt<i32> {
                let $x = pt.x();
                let $y = pt.y();
                Pt::new($ex, $ey)
            }

            /// Translate and add center
            pub fn coords(c: (i32, i32), pt: Pt<i32>) -> Pt<i32> {
                let $x = pt.x();
                let $y = pt.y();
                Pt::new($ex + c.0, $ey + c.1)
            }
        }

        impl Iterator for $o {
            type Item = (i32, i32);

            fn next(&mut self) -> Option<Self::Item> {
                if self.x > self.y {
                    return None;
                }
                let $x = self.x;
                let $y = self.y;
                if let Some(e) = self.e {
                    if self.x + self.c.0 == e.get() {
                        return None;
                    }
                }
                self.x += 1;
                if self.d > 0 {
                    self.y -= 1;
                    self.d += 2 * (self.x - self.y) + 1;
                } else {
                    self.d += 2 * self.x + 1;
                }
                Some(($ex + self.c.0, $ey + self.c.1))
            }
        }
    };
}

bres_oct!(Oct1, 1, x, y, (y, -x), (-y, x));
bres_oct!(Oct2, 2, x, y, (x, -y), (x, -y));
bres_oct!(Oct3, 3, x, y, (-x, -y), (-x, -y));
bres_oct!(Oct4, 4, x, y, (-y, -x), (-y, -x));
bres_oct!(Oct5, 5, x, y, (-y, x), (y, -x));
bres_oct!(Oct6, 6, x, y, (-x, y), (-x, y));
bres_oct!(Oct7, 7, x, y, (x, y), (x, y));
bres_oct!(Oct8, 8, x, y, (y, x), (y, x));

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bres::draw_bres_circle;
    #[test]
    fn bres_macro_oct() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        crate::draw_iter(
            &mut image,
            Oct1::full(crate::RADIUS, crate::CENTER),
            image::Rgba([255, 0, 0, 255]),
        );
        image.save("images/bres_macro_octant.png")
    }
    #[test]
    fn bres_macro_circle() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        draw_bres_circle(
            &mut image,
            crate::RADIUS,
            crate::CENTER,
            image::Rgba([255, 0, 0, 255]),
        );
        image.save("images/bres_iter_circle.png")
    }

    #[test]
    fn pixel_count() {
        let mut image = crate::blank();
        draw_bres_circle(
            &mut image,
            crate::RADIUS,
            crate::CENTER,
            image::Rgba([0, 0, 255, 255]),
        );
        let count = image
            .pixels()
            .filter(|c| **c != image::Rgba([255, 255, 255, 255]))
            .count();
        println!("R={} count={}", crate::RADIUS, count);
    }

    const RAD: f64 = std::f64::consts::PI / 4.0;
    #[test]
    fn accuracy_test() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        let len = 200;

        let correct = image::Rgba([180, 180, 180, 255]);
        let notice = image::Rgba([184, 120, 184, 255]);
        let dark = image::Rgba([0, 0, 0, 255]);
        let incorrect = image::Rgba([255, 0, 0, 255]);

        let mut plots: Vec<(i32, i32)> = Vec::with_capacity(len);
        let mut plots_fp: Vec<(f64, f64)> = Vec::with_capacity(len);

        for i in 0..=len {
            let i = i as f64 / len as f64;
            let a = RAD * i + RAD;
            let p = Pt::from_radian(a, crate::RADIUS, crate::CENTER);
            let x = p.x().round() as i32;
            let y = p.y().round() as i32;
            if plots.len() > 0 {
                if plots[plots.len() - 1].0 == x {
                    // let ny = plots[plots.len() - 1].1;
                    // if ny != y {
                    //     println!("skipping x={} y={} ny={}", x, y, ny);
                    // }
                    continue;
                }
            }
            plots.push((x, y));
            plots_fp.push((p.x(), p.y()));
        }

        let mut ly = crate::RADIUS;
        let (mut px, mut py) = plots.pop().unwrap();
        let mut i = plots.len() - 1;

        for (x, y) in Oct2::full(crate::RADIUS, crate::CENTER) {
            let mut color = correct;
            if x == px {
                color = notice;

                if y != py {
                    color = dark;
                    image.put_pixel(px as u32, py as u32, incorrect);
                    println!(
                        "x={}\ty={}\tpy={}\tox={:.3}\toy={:.3}",
                        x, y, py, plots_fp[i].0, plots_fp[i].1
                    );
                    let cmpx = x.cmp(&(plots_fp[i].0.round() as i32));
                    let cmpy = y.cmp(&(plots_fp[i].1.round() as i32));
                    println!("\tl={}\t\trx={:?}  \try={:?}", ly, cmpx, cmpy);
                    println!("Algo:\t{:?}", Pt::new(px, -py).polar(crate::CENTER.into()));
                    println!("Iter:\t{:?}\n", Pt::new(x, -y).polar(crate::CENTER.into()));
                }
                if i > 0 {
                    if let Some((nx, ny)) = plots.pop() {
                        px = nx;
                        py = ny;
                    }
                    i -= 1;
                }
            }
            image.put_pixel(x as u32, y as u32, color);

            ly = y;
        }

        image.save("images/pixel_test.png")
    }
    #[test]
    fn angle_info() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        let color = image::Rgba([255, 0, 0, 255]);
        let mut iter = Oct8::full(crate::RADIUS, crate::CENTER);
        for (x, y) in &mut iter {
            let pt = Pt::new(x, y);
            let a = pt.angle(crate::CENTER.into());
            println!("x={} y={} pt=({}, {}) angle={:.5}", x, y, pt.x(), pt.y(), a,);
            image.put_pixel(x as u32, y as u32, color);
        }
        image.save("images/angle_test.png")
    }
    #[test]
    fn iter_until() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        let color = image::Rgba([255, 0, 0, 255]);
        // let mut iter = Oct8::new(crate::RADIUS, crate::CENTER);
        let end_theta = std::f64::consts::PI * 2.0 / 8.0 * 6.25;
        let mut iter = Oct4::until(end_theta, crate::RADIUS, crate::CENTER);
        for (x, y) in &mut iter {
            println!("x={} y={}", x, y);
            image.put_pixel(x as u32, y as u32, color);
        }
        image.save("images/iter_until.png")
    }
    #[test]
    fn iter_segment() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        let color = image::Rgba([255, 0, 0, 255]);
        let start_theta = std::f64::consts::PI * 2.0 / 8.0 * 6.25;
        let end_theta = std::f64::consts::PI * 2.0 / 8.0 * 6.5;
        // let mut iter = Oct4::segment(start_theta, end_theta, crate::RADIUS, crate::CENTER);
        let mut iter = Oct5::arc(
            Some(start_theta),
            Some(end_theta),
            crate::RADIUS,
            crate::CENTER,
        );
        for (x, y) in &mut iter {
            image.put_pixel(x as u32, y as u32, color);
        }
        image.save("images/iter_segment.png")
    }
    #[test]
    fn rev_oct() -> Result<(), image::ImageError> {
        let mut image = crate::setup(crate::RADIUS);
        let iter = OctRev::new(crate::RADIUS, crate::CENTER.into());
        crate::draw_iter(&mut image, iter, image::Rgba([255, 0, 0, 255]));
        image.save("images/rev_oct.png")
    }
}
