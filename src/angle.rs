use crate::RADS;

pub(crate) fn angle_to_octant(angle: f64) -> u8 {
    (angle / RADS).floor() as u8 + 1
}

pub(crate) fn octant_start_angle(oct: u8) -> f64 {
    (oct - 1) as f64 * RADS
}
pub(crate) fn octant_end_angle(oct: u8) -> f64 {
    // subtract a *very* tiny amount to prevent moving into the next octant
    oct as f64 * RADS - std::f64::EPSILON * 2.0
}

pub(crate) fn degree_to_radians<T>(degrees: T) -> f64
where
    T: Angle,
{
    (degrees.f64()).to_radians()
}

pub trait Angle {
    fn f64(&self) -> f64;
    fn radians(&self) -> f64 {
        self.f64().to_radians()
    }
}

impl Angle for f64 {
    fn f64(&self) -> f64 {
        *self
    }
}

impl Angle for u16 {
    fn f64(&self) -> f64 {
        (self % 360) as f64
    }
}

impl Angle for u32 {
    fn f64(&self) -> f64 {
        (self % 360) as f64
    }
}

impl Angle for usize {
    fn f64(&self) -> f64 {
        (self % 360) as f64
    }
}

impl Angle for u64 {
    fn f64(&self) -> f64 {
        (self % 360) as f64
    }
}
