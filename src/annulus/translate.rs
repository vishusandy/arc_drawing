use crate::pt::Pt;

pub(super) fn iter_to_real(x: i32, y: i32, oct: u8, c: Pt<i32>) -> Pt<i32> {
    match oct {
        1 => (y + c.x(), -x + c.y()).into(),
        2 => (x + c.x(), -y + c.y()).into(),
        3 => (-x + c.x(), -y + c.y()).into(),
        4 => (-y + c.x(), -x + c.y()).into(),
        5 => (-y + c.x(), x + c.y()).into(),
        6 => (-x + c.x(), y + c.y()).into(),
        7 => (x + c.x(), y + c.y()).into(),
        8 => (y + c.x(), x + c.y()).into(),
        _ => (x + c.x(), y + c.y()).into(),
    }
}
pub(super) fn real_to_iter<T>(mut x: T, mut y: T, oct: u8, c: Pt<T>) -> Pt<T>
where
    T: Copy + std::ops::Neg<Output = T> + std::ops::SubAssign,
{
    x -= c.x();
    y -= c.y();
    match oct {
        1 => Pt::new(-y, x),
        2 => Pt::new(x, -y),
        3 => Pt::new(-x, -y),
        4 => Pt::new(-y, -x),
        5 => Pt::new(y, -x),
        6 => Pt::new(-x, y),
        7 => Pt::new(x, y),
        8 => Pt::new(y, x),
        _ => Pt::new(x, y),
    }
}
