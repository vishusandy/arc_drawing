use crate::lines::LineIter;
use crate::Pt;

#[derive(Clone, Debug)]
pub(super) struct Edge {
    pub(super) top: Pt<i32>,
    pub(super) bot: Pt<i32>,
    pub(super) slope: Slope,
}

impl Edge {
    pub(super) fn new(top: Pt<i32>, bot: Pt<i32>, slope: Slope) -> Self {
        Self { top, bot, slope }
    }
}

#[derive(Clone, Debug)]
pub(super) struct LineIter {
    pub(super) p: Pt<i32>,
    pub(super) it: LineIter,
}

impl LineIter {
    pub(super) fn new(mut it: LineIter) -> Option<Self> {
        it.next().map(|p| Self { p, it })
    }

    pub(super) fn step(&mut self, x: i32) -> Option<Pt<i32>> {
        if x == self.p.x() {
            let pt = self.p;
            if let Some(p) = self.it.next() {
                self.p = p;
            }
            Some(pt)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct Slope {
    slope: f64,
    int: f64,
}

impl Slope {
    pub(super) fn new(a: Pt<i32>, b: Pt<i32>) -> Self {
        let slope = crate::calc_slope(a.x(), a.y(), b.x(), b.y());
        let int = crate::calc_intercept(b.x(), b.y(), slope);

        Self { slope, int }
    }

    pub(super) fn y(&self, x: i32) -> i32 {
        (self.slope * x as f64 + self.int).round() as i32
    }

    #[allow(dead_code)]
    pub(super) fn x(&self, y: i32) -> i32 {
        ((y as f64 - self.int) / self.slope).round() as i32
    }

    pub(super) fn pt_y(&self, x: i32) -> Pt<i32> {
        Pt::new(x, self.y(x))
    }

    pub(super) fn rev_slope_offset(&self, pt: Pt<i32>, offset_y: i32) -> Pt<i32> {
        let slope = -self.slope; // simplify `1 / -(1/slope)` to just `-slope`
        let offset_x = (slope * offset_y as f64).round() as i32;
        Pt::new(pt.x() + offset_x, pt.y() + offset_y)
    }
}
