use crate::{
    error::FacialProcessingError,
    face_processor_trait::FaceProcessorTrait,
    utils::{BoundingBox, Eyes, Point, Rotation},
};
use std::cell::{Cell, RefCell};
use tflite::{
    ops::builtin::BuiltinOpResolver, Error, FlatBufferModel, Interpreter, InterpreterBuilder,
};

pub struct OpenVTFaceProcessor<'a> {
    face_detector_model: RefCell<Interpreter<'a, BuiltinOpResolver>>,
    face_alignment_model: RefCell<Interpreter<'a, BuiltinOpResolver>>,
    eyesolation_model: RefCell<Interpreter<'a, BuiltinOpResolver>>,
    face_detector_confidence: Cell<f32>,
    face_detector_cores: Cell<i16>,
}

impl OpenVTFaceProcessor {
    pub fn new() -> Result<Self, FacialProcessingError> {
        let default_resolver_face = BuiltinOpResolver::default();
        let default_resolver_land = BuiltinOpResolver::default();
        let default_resolver_iris = BuiltinOpResolver::default();

        // load the face detector
        let face_detector_load = match FlatBufferModel::build_from_buffer(
            include_bytes!("weights/weights/RFB-320.tflite").to_vec(),
        ) {
            Ok(m) => m,
            Err(why) => return Err(FacialProcessingError::from(why)),
        };
        let face_detector_model =
            match InterpreterBuilder::new(face_detector_model, default_resolver_face) {
                Ok(m) => match m.build() {
                    Ok(int) => int,
                    Err(why) => return Err(FacialProcessingError(why)),
                },
                Err(why) => return Err(FacialProcessingError(why)),
            };

        // load landmark detector
        let face_alignment_load = match FlatBufferModel::build_from_buffer(
            include_bytes!("weights/weights/coor_2d106.tflite").to_vec(),
        ) {
            Ok(m) => m,
            Err(why) => return Err(FacialProcessingError::from(why)),
        };
        let face_alignment_model =
            match InterpreterBuilder::new(face_alignment_load, default_resolver_land) {
                Ok(m) => match m.build() {
                    Ok(int) => int,
                    Err(why) => return Err(FacialProcessingError(why)),
                },
                Err(why) => return Err(FacialProcessingError(why)),
            };

        // load iris isloator
        let eyesolator_load = match FlatBufferModel::build_from_buffer(
            include_bytes!("weights/weights/iris_localization.tflite").to_vec(),
        ) {
            Ok(m) => m,
            Err(why) => return Err(FacialProcessingError::from(why)),
        };
        let eyesolator_model = match InterpreterBuilder::new(eyesolator_load, default_resolver_iris)
        {
            Ok(m) => match m.build() {
                Ok(int) => int,
                Err(why) => return Err(FacialProcessingError(why)),
            },
            Err(why) => return Err(FacialProcessingError(why)),
        };
    }
    fn prepare() {}
}

impl FaceProcessorTrait for OpenVTFaceProcessor {
    fn init(&self, cpu: i16, confidence: f32) -> Result<(), FacialProcessingError> {
        unimplemented!()
    }

    fn get_face_detections(&self, data: &Vec<u8>) -> Vec<BoundingBox<f32>> {
        unimplemented!()
    }

    fn get_face_landmarks(&self, data: &Vec<u8>, bbox: BoundingBox<f32>) -> [Point<f32>; 68] {
        unimplemented!()
    }

    fn get_iris_locations(&self, data: &Vec<u8>, irises: &[Point<f32>; 68]) -> [Eyes; 2] {
        unimplemented!()
    }

    fn solve_pose(&self, pose: &[Point<f32>; 68]) -> Rotation {
        unimplemented!()
    }
}
