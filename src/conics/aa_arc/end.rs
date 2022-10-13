use crate::Pt;

/// Represents an end point in the fast direction.
#[derive(Copy, Clone, Debug)]
pub(super) enum End {
    X(f64),
    Y(f64),
}
impl End {
    /// Create a new end point from a point.  If `x <= y` then it will use the x coordinate,
    /// otherwise it will use the y coordinate.
    pub(super) fn new(p: Pt<f64>) -> Self {
        if p.x() <= p.y() {
            Self::X(p.x())
        } else {
            Self::Y(p.y())
        }
    }

    /// Check if the end point has been reached
    #[allow(dead_code)]
    pub(super) fn r#match(&self, p: Pt<f64>) -> bool {
        match self {
            Self::X(x) => *x <= p.x,
            Self::Y(y) => *y >= p.y,
        }
    }

    /// Check if an X end point has been reached
    pub(super) fn match_x(&self, p: f64) -> bool {
        match self {
            Self::X(x) => *x <= p,
            _ => false,
        }
    }

    /// Check if an Y end point has been reached
    pub(super) fn match_y(&self, p: f64) -> bool {
        match self {
            Self::Y(y) => *y >= p,
            _ => false,
        }
    }
}
