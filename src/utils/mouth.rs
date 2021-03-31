use crate::utils::misc::Point;

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Mouth {
    pub points_outer: [Point; 12],
    pub poitns_inner: [Point; 8],
}
