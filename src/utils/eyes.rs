use crate::utils::misc::{LeftRight, Point};

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Eyes {
    points: [Point; 6],
    leftright: LeftRight,
    estimate_open: f64,
    iris_left_right: f64,
    iris_up_down: f64,
}
impl Eyes {
    pub fn from_points(data: &[Point; 6]) -> Self {}
}
