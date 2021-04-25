use crate::{error::FacialProcessingError, utils::{face::FaceLandmark, misc::{BoundingBox, EulerAngles}}};
use image::{ImageBuffer, Rgb};

pub trait FaceProcessorTrait {
    fn init(&self, cpu: i16, confidence: f32) -> Result<(), FacialProcessingError>;
    fn get_face_detections(&self, data: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<BoundingBox>;
    fn get_face_landmark(
        &self,
        data: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        bbox: BoundingBox,
    ) -> FaceLandmark;

    fn get_pnp_forward(&self,
        data: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        landmark: FaceLandmark) -> EulerAngles;
    
        
}
