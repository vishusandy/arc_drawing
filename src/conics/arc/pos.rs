use super::Bounds;
use crate::Pt;

#[derive(Clone, Debug)]
pub(super) struct Pos {
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) d: i32,
    pub(super) oct: u8,
    pub(super) ex: Option<i32>,
}

impl Pos {
    pub(super) fn new(oct: u8, bounds: Bounds, r: i32, c: Pt<i32>) -> Self {
        let (Pt { x, y }, d) = match bounds.start {
            None => (Pt::new(0, r), 1 - r),
            Some(a) => {
                let pt = Pt::from_radian(a, r, c).real_to_iter(oct, c.into());
                (pt.i32(), crate::calc_error(pt, r))
            }
        };

        let ex = bounds.end.map(|a| {
            Pt::from_radian(a, r, c)
                .real_to_iter(oct, c.into())
                .x()
                .round() as i32
        });

        Self { x, y, d, oct, ex }
    }

    pub(super) fn stop(&self) -> bool {
        self.x >= self.ex.map(|ex| ex + 1).unwrap_or(self.y)
    }

    pub(super) fn start(oct: u8, r: i32) -> Self {
        Self {
            x: 0,
            y: r,
            d: 1 - r,
            oct,
            ex: None,
        }
    }

    pub(super) fn inc(&mut self) {
        self.x += 1;
        if self.d > 0 {
            self.y -= 1;
            self.d += 2 * (self.x - self.y) + 1;
        } else {
            self.d += 2 * self.x + 1;
        }
    }
}
