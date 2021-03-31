use crate::error::FacialProcessingError;
use crate::utils_post::BoundingBox;
use dlib_face_recognition::{
    FaceDetector, FaceDetectorTrait, ImageMatrix, LandmarkPredictor, LandmarkPredictorTrait,
    Rectangle,
};
use image::{ImageBuffer, Rgb};
use std::path::Path;
use crate::utils::misc::BoundingBox;

pub struct DlibProcessor {
    face_detector: FaceDetector,
    landmark_detector: LandmarkPredictor,
}

impl DlibProcessor {
    pub fn from_file<P: AsRef<Path>>(landmark_detector: P) -> Result<Self, FacialProcessingError> {
        let landmark = match LandmarkPredictor::new(landmark_detector) {
            Ok(land) => land,
            Err(why) => Err(FacialProcessingError::InitializeError(why)),
        };
        Ok(DlibProcessor {
            face_detector: FaceDetector::new(),
            landmark_detector: landmark,
        })
    }
    fn detect_faces(&self, data: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<BoundingBox> {
        let mut boxes: Vec<BoundingBox> = vec![];
        let image = ImageMatrix::from_image(data);
        for rect in self.face_detector.face_locations(&image).iter() {
            boxes.push(BoundingBox::from(*rect))
        }
        boxes
    }
    fn landmark_faces(&self, data: &ImageBuffer<Rgb<u8>, Vec<u8>>, bboxes: Vec<BoundingBox>) ->
}
