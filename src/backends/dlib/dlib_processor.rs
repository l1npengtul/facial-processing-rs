use crate::{
    error::FacialProcessingError,
    face_processor_trait::FaceProcessorTrait,
    utils::{
        eyes::Eye,
        face::FaceLandmark,
        misc::{BoundingBox, EulerAngles, Point2D},
    },
};
use dlib_face_recognition::{
    FaceDetector, FaceDetectorTrait, ImageMatrix, LandmarkPredictor, LandmarkPredictorTrait,
    Rectangle,
};
use image::{ImageBuffer, Rgb};
use std::path::Path;

pub struct DLibProcessor {
    face_detector: FaceDetector,
    landmark_detector: LandmarkPredictor,
}

impl DLibProcessor {
    pub fn new<P: AsRef<Path>>(landmark_detector: P) -> Result<Self, FacialProcessingError> {
        let landmark = match LandmarkPredictor::new(landmark_detector) {
            Ok(land) => land,
            Err(why) => return Err(FacialProcessingError::InitializeError(why)),
        };
        Ok(DLibProcessor {
            face_detector: FaceDetector::new(),
            landmark_detector: landmark,
        })
    }

    pub fn detect_faces(&self, data: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<BoundingBox> {
        self.detect_faces_imagematrix(&ImageMatrix::from_image(data))
    }

    pub fn detect_faces_imagematrix(&self, data: &ImageMatrix) -> Vec<BoundingBox> {
        let mut boxes: Vec<BoundingBox> = vec![];
        for rect in self.face_detector.face_locations(data).iter() {
            boxes.push(BoundingBox::from(*rect))
        }
        boxes
    }

    pub fn landmark_faces(
        &self,
        data: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        bbox: BoundingBox,
    ) -> FaceLandmark {
        self.landmark_faces_imagematrix(&ImageMatrix::from_image(data), bbox)
    }

    pub fn landmark_faces_imagematrix(
        &self,
        data: &ImageMatrix,
        bbox: BoundingBox,
    ) -> FaceLandmark {
        let landmark = self.landmark_detector.face_landmarks(data, &bbox.into());
        FaceLandmark::from_dlib(bbox, landmark.to_vec())
    }
}

impl FaceProcessorTrait for DLibProcessor {
    fn init(&self, cpu: i16, confidence: f32) -> Result<(), FacialProcessingError> {
        // nothing to-do here.
        Ok(())
    }

    fn get_face_detections(&self, data: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<BoundingBox> {
        self.detect_faces(data)
    }

    fn get_face_landmark(
        &self,
        data: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        bbox: BoundingBox,
    ) -> FaceLandmark {
        self.landmark_faces(data, bbox)
    }

    fn get_pnp_forward(&self,
        data: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        landmark: FaceLandmark) -> EulerAngles {
            todo!()
        }
}
