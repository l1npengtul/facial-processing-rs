use crate::utils::eyes::Eye;
use crate::utils::face::FaceLandmark;
use crate::utils::misc::{EulerAngles, Point2D};
use crate::{
    error::FacialProcessingError, face_processor_trait::FaceProcessorTrait,
    utils::misc::BoundingBox,
};
use image::{ImageBuffer, Rgb};
use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    path::Path,
};
use tflite::{
    ops::builtin::BuiltinOpResolver, Error, FlatBufferModel, Interpreter, InterpreterBuilder,
};

pub struct OpenVTFaceProcessor<'a> {
    face_detector_model: RefCell<Interpreter<'a, BuiltinOpResolver>>,
    face_alignment_model: RefCell<Interpreter<'a, BuiltinOpResolver>>,
    eyesolation_model: RefCell<Interpreter<'a, BuiltinOpResolver>>,
    face_detector_confidence: Cell<f32>,
    face_detector_cores: Cell<i16>,
    face_eyesolator_threashold: Cell<f64>,
}

impl OpenVTFaceProcessor {
    pub fn new<P: AsRef<Path>>(
        face_detector_path: P,
        face_alignment_path: P,
        face_eyesolator_path: P,
    ) -> Result<Self, FacialProcessingError> {
        let default_resolver_face = BuiltinOpResolver::default();
        let default_resolver_land = BuiltinOpResolver::default();
        let default_resolver_iris = BuiltinOpResolver::default();

        // load the face detector
        let face_detector_load = match FlatBufferModel::build_from_file(face_detector_path) {
            Ok(m) => m,
            Err(why) => return Err(FacialProcessingError::from(why)),
        };
        let face_detector_model =
            match InterpreterBuilder::new(face_detector_load, default_resolver_face) {
                Ok(m) => match m.build() {
                    Ok(int) => int,
                    Err(why) => return Err(FacialProcessingError(why)),
                },
                Err(why) => return Err(FacialProcessingError(why)),
            };

        // load landmark detector
        let face_alignment_load = match FlatBufferModel::build_from_file(face_alignment_path) {
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
        let face_eyesolator_load = match FlatBufferModel::build_from_file(face_eyesolator_path) {
            Ok(m) => m,
            Err(why) => return Err(FacialProcessingError::from(why)),
        };
        let face_eyesolator_model =
            match InterpreterBuilder::new(face_eyesolator_load, default_resolver_iris) {
                Ok(m) => match m.build() {
                    Ok(int) => int,
                    Err(why) => return Err(FacialProcessingError(why)),
                },
                Err(why) => return Err(FacialProcessingError(why)),
            };

        Ok(OpenVTFaceProcessor {
            face_detector_model: RefCell::new(face_detector_model),
            face_alignment_model: RefCell::new(face_alignment_model),
            eyesolation_model: RefCell::new(face_eyesolator_model),
            face_detector_confidence: Cell::new(0.85_f32),
            face_detector_cores: Cell::new(1),
            face_eyesolator_threashold: Cell::new(0.2),
        })
    }

    fn prepare() {}
}

impl FaceProcessorTrait for OpenVTFaceProcessor {
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
