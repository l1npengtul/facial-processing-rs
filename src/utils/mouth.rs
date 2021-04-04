use crate::utils::misc::Point2D;

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Mouth {
    pub points_outer: [Point2D; 12],
    pub points_inner: [Point2D; 8],
}
