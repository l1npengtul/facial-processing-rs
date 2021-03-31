use crate::utils::misc::Point;

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct FaceLandmarks {
    all: [Point; 68],
    chin: [Point; 17],
}
