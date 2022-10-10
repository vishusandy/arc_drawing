use super::Edge;

#[derive(Clone, Debug, Default)]
pub(super) struct Bounds {
    pub(super) start: Option<f64>,
    pub(super) end: Option<f64>,
}

impl Bounds {
    pub(super) fn new(start: Option<f64>, end: Option<f64>) -> Self {
        Self { start, end }
    }

    pub(super) fn start_bounds(start_edge: &Edge, end_edge: &Edge, revisit: bool) -> Self {
        let start = Some(start_edge.angle);

        let end = match start_edge.oct == end_edge.oct && !revisit {
            true => Some(end_edge.angle),
            false => None,
        };

        match start_edge.oct % 2 == 0 {
            true => Bounds::new(end, start),
            false => Bounds::new(start, end),
        }
    }

    #[allow(clippy::self_named_constructors)]
    pub(super) fn bounds(oct: u8, start_edge: &Edge, end_edge: &Edge, revisit: bool) -> Self {
        if oct != end_edge.oct {
            return Bounds::default();
        }

        let start = match oct == start_edge.oct && start_edge.oct != end_edge.oct {
            true => Some(start_edge.angle),
            false => None,
        };

        let end = match oct == end_edge.oct && !revisit {
            true => Some(end_edge.angle),
            false => None,
        };

        match oct % 2 == 0 {
            true => Bounds::new(end, start),
            false => Bounds::new(start, end),
        }
    }
}
