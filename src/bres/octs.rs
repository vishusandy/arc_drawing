pub(super) mod bres_to {
    pub(in super::super) fn o1<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.1, -c.0)
    }
    pub(in super::super) fn o2<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.0, -c.1)
    }
    pub(in super::super) fn o3<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.0, -c.1)
    }
    pub(in super::super) fn o4<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.1, -c.0)
    }
    pub(in super::super) fn o5<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.1, c.0)
    }
    pub(in super::super) fn o6<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (-c.0, c.1)
    }
    pub(in super::super) fn o7<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        c
    }
    pub(in super::super) fn o8<T: std::ops::Neg<Output = T>>(c: (T, T)) -> (T, T) {
        (c.1, c.0)
    }
}
