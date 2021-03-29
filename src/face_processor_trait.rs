use crate::{
    error::FacialProcessingError,
    utils::{BoundingBox, Eyes, Point, Rotation, SupportedProcesses},
};
use std::{ops::Deref, path::Path};

pub trait FaceProcessorTrait {
    fn init(&self, cpu: i16, confidence: f32) -> Result<(), FacialProcessingError>;
    fn get_face_detections(&self, data: &Vec<u8>) -> Vec<BoundingBox<f32>>;
    fn get_face_landmarks(&self, data: &Vec<u8>, bbox: BoundingBox<f32>) -> [Point<f32>; 68];
    fn get_iris_locations(&self, data: &Vec<u8>, irises: &[Point<f32>; 68]) -> [Eyes; 2];
    fn solve_pose(&self, pose: &[Point<f32>; 68]) -> Rotation;
}
