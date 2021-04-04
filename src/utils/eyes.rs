use crate::utils::face::FaceLandmark;
use crate::utils::misc::{LeftRight, Point2D};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct RawEye {
    points: [Point2D; 6],
    leftright: LeftRight,
}
impl RawEye {
    pub fn new(data: FaceLandmark, left: LeftRight) -> Self {
        RawEye {
            points: data.eye_landmarks(left),
            leftright: left,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Eye {
    points: RawEye,
    open_ratio: f64,
    normalize_ratio: f64,
}
impl Eye {
    pub fn from_points(data: FaceLandmark) -> Self {}
}
