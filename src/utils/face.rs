use crate::utils::misc::{BoundingBox, LeftRight, Point2D};
use dlib_face_recognition::Point;

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct FaceLandmark {
    all: Vec<Point2D>,
    bbox: BoundingBox,
}
impl FaceLandmark {
    #[cfg(feature = "dlib")]
    pub fn from_dlib(bbox: BoundingBox, landmarks: Vec<Point>) -> FaceLandmark {
        let mut all = vec![];
        for pt in landmarks {
            all.push(Point2D::from(pt))
        }
        FaceLandmark { all, bbox }
    }

    pub fn landmarks(&self) -> Vec<Point2D> {
        self.all.clone()
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bbox
    }

    pub fn eye_landmarks(&self, side: LeftRight) -> [Point2D; 6] {
        match side {
            LeftRight::Left => [
                *self.all.get(36).unwrap(),
                *self.all.get(37).unwrap(),
                *self.all.get(38).unwrap(),
                *self.all.get(39).unwrap(),
                *self.all.get(40).unwrap(),
                *self.all.get(41).unwrap(),
            ],
            LeftRight::Right => [
                *self.all.get(42).unwrap(),
                *self.all.get(43).unwrap(),
                *self.all.get(44).unwrap(),
                *self.all.get(45).unwrap(),
                *self.all.get(46).unwrap(),
                *self.all.get(47).unwrap(),
            ],
        }
    }

    pub fn eyebrow_landmarks(&self, side: LeftRight) -> [Point2D; 5] {
        match side {
            LeftRight::Left => [
                *self.all.get(17).unwrap(),
                *self.all.get(18).unwrap(),
                *self.all.get(19).unwrap(),
                *self.all.get(20).unwrap(),
                *self.all.get(21).unwrap(),
            ],
            LeftRight::Right => [
                *self.all.get(22).unwrap(),
                *self.all.get(23).unwrap(),
                *self.all.get(24).unwrap(),
                *self.all.get(25).unwrap(),
                *self.all.get(26).unwrap(),
            ],
        }
    }

    pub fn mouth_landmarks(&self) -> ([Point2D; 8], [Point2D; 12]) {
        let mouth_inner_parties = [
            *self.all.get(60).unwrap(),
            *self.all.get(61).unwrap(),
            *self.all.get(62).unwrap(),
            *self.all.get(63).unwrap(),
            *self.all.get(64).unwrap(),
            *self.all.get(65).unwrap(),
            *self.all.get(66).unwrap(),
            *self.all.get(67).unwrap(),
        ];
        let mouth_outer_parties = [
            *self.all.get(48).unwrap(),
            *self.all.get(49).unwrap(),
            *self.all.get(50).unwrap(),
            *self.all.get(51).unwrap(),
            *self.all.get(52).unwrap(),
            *self.all.get(53).unwrap(),
            *self.all.get(54).unwrap(),
            *self.all.get(55).unwrap(),
            *self.all.get(56).unwrap(),
            *self.all.get(57).unwrap(),
            *self.all.get(58).unwrap(),
            *self.all.get(59).unwrap(),
        ];
        (mouth_inner_parties, mouth_outer_parties)
    }

    pub fn nose_landmarks(&self) -> ([Point2D; 4], [Point2D; 5]) {
        let nose_line = [
            *self.all.get(29).unwrap(),
            *self.all.get(30).unwrap(),
            *self.all.get(31).unwrap(),
            *self.all.get(32).unwrap(),
        ];
        let nose_bottom = [
            *self.all.get(31).unwrap(),
            *self.all.get(32).unwrap(),
            *self.all.get(33).unwrap(),
            *self.all.get(34).unwrap(),
            *self.all.get(35).unwrap(),
        ];
        (nose_line, nose_bottom)
    }

    pub fn chin_landmarks(&self) -> [Point2D; 17] {
        [
            *self.all.get(0).unwrap(),
            *self.all.get(1).unwrap(),
            *self.all.get(2).unwrap(),
            *self.all.get(3).unwrap(),
            *self.all.get(4).unwrap(),
            *self.all.get(5).unwrap(),
            *self.all.get(6).unwrap(),
            *self.all.get(7).unwrap(),
            *self.all.get(8).unwrap(),
            *self.all.get(9).unwrap(),
            *self.all.get(10).unwrap(),
            *self.all.get(11).unwrap(),
            *self.all.get(12).unwrap(),
            *self.all.get(13).unwrap(),
            *self.all.get(14).unwrap(),
            *self.all.get(15).unwrap(),
            *self.all.get(16).unwrap(),
        ]
    }
}

impl IntoIterator for FaceLandmark {
    type Item = Point2D;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.all.into_iter()
    }
}
