use crate::{calc_error, Pt};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug)]
pub(super) struct Pos {
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) d: i32,  // decision parameter
    pub(super) ex: i32, // ending x coordinate
    pub(super) ey: i32, // ending y coordinate
    pub(super) r: i32,
}
impl Pos {
    pub(super) fn new(start: f64, end: f64, oct: u8, r: i32, c: Pt<i32>) -> Self {
        let mut start = Pt::from_radian(start, r, c).real_to_iter(oct, c.into());
        let mut end = Pt::from_radian(end, r, c).real_to_iter(oct, c.into());
        let Pt { mut x, mut y } = start.i32();
        let Pt {
            x: mut ex,
            y: mut ey,
        } = end.i32();
        if oct % 2 == 0 {
            std::mem::swap(&mut start, &mut end);
            std::mem::swap(&mut x, &mut ex);
            std::mem::swap(&mut y, &mut ey);
        }
        let d = calc_error(start, r);
        Self { x, y, d, ex, ey, r }
    }

    /// Get `self.y` when `self.x` is the same as the specified `x`
    pub(super) fn matching_y(&self, x: i32) -> Option<i32> {
        if x == self.x {
            Some(self.y)
        } else {
            None
        }
    }

    pub(super) fn inc(&mut self) {
        if self.x >= self.ex {
            return;
        }
        self.x += 1;
        if self.d > 0 {
            self.y -= 1;
            self.d += 2 * (self.x - self.y) + 1;
        } else {
            self.d += 2 * self.x + 1;
        }
    }
}
