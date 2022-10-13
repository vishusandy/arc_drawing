use crate::pt::Pt;

pub(crate) fn iter_to_real(x: i32, y: i32, oct: u8, c: Pt<i32>) -> Pt<i32> {
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
