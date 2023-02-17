use super::Edge;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default)]
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

        let end = if start_edge.oct == end_edge.oct && !revisit {
            Some(end_edge.angle)
        } else {
            None
        };

        if start_edge.oct % 2 == 0 {
            Bounds::new(end, start)
        } else {
            Bounds::new(start, end)
        }
    }

    pub(super) fn bounds_from_edges(
        oct: u8,
        start_edge: &Edge,
        end_edge: &Edge,
        revisit: bool,
    ) -> Self {
        if oct != end_edge.oct {
            return Bounds::default();
        }

        let start = if oct == start_edge.oct && start_edge.oct != end_edge.oct {
            Some(start_edge.angle)
        } else {
            None
        };

        let end = if oct == end_edge.oct && !revisit {
            Some(end_edge.angle)
        } else {
            None
        };

        if oct % 2 == 0 {
            Bounds::new(end, start)
        } else {
            Bounds::new(start, end)
        }
    }
}
