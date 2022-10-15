#[derive(Clone, Debug)]
pub(super) struct Edge {
    pub(super) angle: f64,
    pub(super) oct: u8,
    pub(super) slope: f64,
    pub(super) int: i32, // intercept
}

impl Edge {
    pub(super) fn blank(angle: f64) -> Self {
        Self {
            angle,
            oct: crate::angle::angle_to_octant(angle),
            slope: 0.0,
            int: 0,
        }
    }

    pub(super) fn set_slope(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.slope = crate::calc_slope(x1, y1, x2, y2);
        self.int = (self.slope * (-x1 as f64) + y1 as f64).round() as i32;
    }

    pub(super) fn line(&self) -> (f64, i32) {
        (self.slope, self.int)
    }

    pub(super) fn slope(&self) -> f64 {
        self.slope
    }

    pub(super) fn int(&self) -> i32 {
        self.int
    }
}

pub(super) fn calc_line(slope: f64, int: i32, x: i32) -> i32 {
    (x as f64 * slope).round() as i32 + int
}
