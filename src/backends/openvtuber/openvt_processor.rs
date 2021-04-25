use crate::{error::FacialProcessingError, face_processor_trait::FaceProcessorTrait, utils::{eyes::Eye, face::FaceLandmark, misc::{BoundingBox, EulerAngles, Point2D}}};
use image::{ImageBuffer, Rgb};
use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    path::Path,
};

pub struct OpenVTFaceProcessor {
}

impl OpenVTFaceProcessor {
    pub fn new()
}

impl FaceProcessorTrait for OpenVTFaceProcessor{
    fn init(&self, cpu: i16, confidence: f32) -> Result<(), FacialProcessingError> {
        todo!()
    }

    fn get_face_detections(&self, data: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<BoundingBox> {
        todo!()
    }

    fn get_face_landmark(
        &self,
        data: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        bbox: BoundingBox,
    ) -> FaceLandmark {
        todo!()
    }
}
