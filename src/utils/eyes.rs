use crate::utils::misc::{LeftRight, Point};

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Eyes {
    points: [Point; 6],
    leftright: LeftRight,
    open_ratio: f64,
    normalize_ratio: f64,
}
impl Eyes {
    pub fn from_points(data: &[Point; 6]) -> Self {}
}
