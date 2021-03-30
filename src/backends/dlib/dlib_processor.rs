use dlib_face_recognition::{
    FaceDetector, FaceDetectorTrait, LandmarkPredictor, LandmarkPredictorTrait, Rectangle,
};

pub struct DlibProcessor {
    face_detector: FaceDetector,
    landmark_detector: LandmarkPredictor,
}

impl DlibProcessor {
    pub fn new() -> Self {
        DlibProcessor {
            face_detector: FaceDetector::default(),
            landmark_detector: LandmarkPredictor::default(),
        }
    }
}
