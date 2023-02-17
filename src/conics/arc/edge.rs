#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub(super) struct Edge {
    pub(super) angle: f64,
    pub(super) oct: u8,
}

impl Edge {
    pub(super) fn new(angle: f64, oct: u8) -> Self {
        Edge { angle, oct }
    }
}
