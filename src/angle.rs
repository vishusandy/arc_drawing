use crate::RADS;

#[inline]
pub(crate) fn angle_to_octant(angle: f64) -> u8 {
    (angle / RADS).floor() as u8 + 1
}

#[allow(dead_code)]
#[inline]
pub(crate) fn rev_angle(angle: f64, oct: u8) -> f64 {
    RADS - (angle - octant_start_angle(oct))
}

#[inline]
pub(crate) fn octant_start_angle(oct: u8) -> f64 {
    (oct - 1) as f64 * RADS
}

#[inline]
pub(crate) fn octant_end_angle(oct: u8) -> f64 {
    // subtract a *very* tiny amount to prevent moving into the next octant
    oct as f64 * RADS - crate::TINY
}

#[inline]
pub(crate) fn normalize(angle: f64) -> f64 {
    use crate::PI2;
    (angle % PI2 + PI2) % PI2
}

#[inline]
pub(crate) fn angle_to_quad(angle: f64) -> u8 {
    (angle / crate::QUAD).floor() as u8 + 1
}

pub trait Angle {
    fn f64(&self) -> f64;
    fn radians(&self) -> f64 {
        self.f64().to_radians()
    }
}
impl Angle for f32 {
    fn f64(&self) -> f64 {
        *self as f64
    }
    fn radians(&self) -> f64 {
        *self as f64
    }
}

impl Angle for f64 {
    fn f64(&self) -> f64 {
        *self
    }
    fn radians(&self) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    const RADS_F32: f32 = std::f32::consts::PI / 4.0;
    #[test]
    fn angles() {
        assert_eq!(180u16.radians(), 4.0 * RADS);
        assert_eq!(180u32.radians(), 4.0 * RADS);
        assert_eq!(180u64.radians(), 4.0 * RADS);
        assert_eq!(180usize.radians(), 4.0 * RADS);
        assert!(((4.0f32 * RADS_F32).radians() - 4.0 * RADS) <= std::f32::EPSILON as f64);
    }
}
