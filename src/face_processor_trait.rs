use crate::{
    error::FacialProcessingError,
    utils::{
        eyes::Eye,
        face::FaceLandmark,
        misc::{BoundingBox, EulerAngles, Point2D},
    },
};
use image::{ImageBuffer, Rgb};
use std::{ops::Deref, path::Path};

pub trait FaceProcessorTrait {
    fn init(&self, cpu: i16, confidence: f32) -> Result<(), FacialProcessingError>;
    fn get_face_detections(&self, data: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<BoundingBox>;
    fn get_face_landmark(
        &self,
        data: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        bbox: BoundingBox,
    ) -> Vec<FaceLandmark>;
}
