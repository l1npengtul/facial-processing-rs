use std::{
    ops::Deref,
    path::Path
};
use crate::{
    error::FacialProcessingError,
    utils::{
        SupportedProcesses,
        BoundingBox,
        Point,
        Eyes,
        Rotation
    }
};

pub trait FaceProcessor {
    fn init() -> Result<(), FacialProcessingError>;
    fn get_face_detections() -> Vec<BoundingBox<f32>>;
    fn get_face_landmarks(bbox: BoundingBox<f32>) -> [Point<f32>; 68];
    fn get_iris_locations(irises: &[Point<f32>; 68]) -> [Eyes; 2];
    fn solve_pose(pose: &[Point<f32>; 68]) -> Rotation;
}
