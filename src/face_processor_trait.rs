use crate::{
    error::FacialProcessingError,
    utils::{
        eyes::Eyes,
        misc::{BoundingBox, Point, Rotation},
    },
};
use image::{ImageBuffer, Rgb};
use std::{ops::Deref, path::Path};

pub trait FaceProcessorTrait {
    fn init(&self, cpu: i16, confidence: f32) -> Result<(), FacialProcessingError>;
    fn get_face_detections(&self, data: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<BoundingBox>;
    fn get_face_landmarks(
        &self,
        data: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        bbox: BoundingBox,
    ) -> [Point; 68];
    fn get_iris_locations(
        &self,
        data: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        irises: &[Point; 68],
    ) -> [Eyes; 2];
    fn solve_pose(&self, pose: &[Point; 68]) -> Rotation;
}
