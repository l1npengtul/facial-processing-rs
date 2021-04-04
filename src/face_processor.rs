// TODO: Unified detector that abstracts over a backend here

use crate::{
    backends::{
        dlib::dlib_processor::DLibProcessor, openvtuber::openvt_processor::OpenVTFaceProcessor,
    },
    error::FacialProcessingError,
    face_processor_trait::FaceProcessorTrait,
    utils::{
        face::FaceLandmark,
        misc::{BackendProviders, ImageScale},
    },
};
use getset::{CopyGetters, Getters, MutGetters};
use image::{imageops::FilterType, ImageBuffer, Rgb};
use std::cell::{Cell, RefCell};

pub struct FaceProcessorBuilder {
    backend: BackendProviders,
    desired_threads: Option<i16>,
    do_eye_calculations: bool,
    do_mouth_calculations: bool,
    eye_blink_ratio: f64,
    input_image_x: u32,
    input_image_y: u32,
    image_scale: Option<ImageScale>,
}

impl FaceProcessorBuilder {
    pub fn new() -> Self {
        FaceProcessorBuilder {
            backend: BackendProviders::DLib,
            desired_threads: None,
            do_eye_calculations: true,
            do_mouth_calculations: true,
            eye_blink_ratio: 0.2,
            input_image_x: 640,
            input_image_y: 480,
            image_scale: None,
        }
    }

    pub fn with_eye_calibration(self, eye_blink_ratio: f64) -> Self {
        FaceProcessorBuilder {
            backend: self.backend,
            desired_threads: self.desired_threads,
            do_eye_calculations: self.do_eye_calculations,
            do_mouth_calculations: self.do_mouth_calculations,
            eye_blink_ratio,
            input_image_x: self.input_image_x,
            input_image_y: self.input_image_y,
            image_scale: self.image_scale,
        }
    }

    pub fn with_image_scale(self, target_x: u32, target_y: u32, method: FilterType) -> Self {
        let image_scale = Some(ImageScale {
            target_x,
            target_y,
            method,
        });
        FaceProcessorBuilder {
            backend: self.backend,
            desired_threads: self.desired_threads,
            do_eye_calculations: self.do_eye_calculations,
            do_mouth_calculations: self.do_mouth_calculations,
            eye_blink_ratio: self.eye_blink_ratio,
            input_image_x: self.input_image_x,
            input_image_y: self.input_image_y,
            image_scale,
        }
    }

    pub fn with_input(self, input_image_x: u32, input_image_y: u32) -> Self {
        FaceProcessorBuilder {
            backend: self.backend,
            desired_threads: self.desired_threads,
            do_eye_calculations: self.do_eye_calculations,
            do_mouth_calculations: self.do_mouth_calculations,
            eye_blink_ratio: self.eye_blink_ratio,
            input_image_x,
            input_image_y,
            image_scale: self.image_scale,
        }
    }

    pub fn with_backend(self, backend: BackendProviders) -> Self {
        FaceProcessorBuilder {
            backend,
            desired_threads: self.desired_threads,
            do_eye_calculations: self.do_eye_calculations,
            do_mouth_calculations: self.do_mouth_calculations,
            eye_blink_ratio: self.eye_blink_ratio,
            input_image_x: self.input_image_x,
            input_image_y: self.input_image_y,
            image_scale: self.image_scale,
        }
    }

    pub fn with_desired_threads(self, threads: i16) -> Self {
        let desired_threads = Some(threads);
        FaceProcessorBuilder {
            backend: self.backend,
            desired_threads,
            do_eye_calculations: self.do_eye_calculations,
            do_mouth_calculations: self.do_mouth_calculations,
            eye_blink_ratio: self.eye_blink_ratio,
            input_image_x: self.input_image_x,
            input_image_y: self.input_image_y,
            image_scale: self.image_scale,
        }
    }

    pub fn with_eye_calculations(self, do_eye_calculations: bool) -> Self {
        FaceProcessorBuilder {
            backend: self.backend,
            desired_threads: self.desired_threads,
            do_eye_calculations,
            do_mouth_calculations: self.do_mouth_calculations,
            eye_blink_ratio: self.eye_blink_ratio,
            input_image_x: self.input_image_x,
            input_image_y: self.input_image_y,
            image_scale: self.image_scale,
        }
    }

    pub fn with_mouth_calculations(self, do_mouth_calculations: bool) -> Self {
        FaceProcessorBuilder {
            backend: self.backend,
            desired_threads: self.desired_threads,
            do_eye_calculations: self.do_eye_calculations,
            do_mouth_calculations,
            eye_blink_ratio: self.eye_blink_ratio,
            input_image_x: self.input_image_x,
            input_image_y: self.input_image_y,
            image_scale: self.image_scale,
        }
    }

    pub fn build(self) -> Result<FaceProcessor, FacialProcessingError> {
        let backend_held: Box<dyn FaceProcessorTrait> = match self.backend.clone() {
            BackendProviders::OpenVTuber {
                face_detector_path,
                face_alignment_path,
                face_eyesolator_path,
            } => {
                match OpenVTFaceProcessor::new(
                    face_detector_path,
                    face_alignment_path,
                    face_eyesolator_path,
                ) {
                    Ok(process) => Box::new(process),
                    Err(why) => return Err(why),
                }
            }
            BackendProviders::DLib {
                face_alignment_path,
            } => match DLibProcessor::new(face_alignment_path) {
                Ok(process) => Box::new(process),
                Err(why) => return Err(why),
            },
        };

        Ok(FaceProcessor {
            backend_setting: self.backend,
            backend_held,
            do_eye_calculations: self.do_eye_calculations,
            do_mouth_calculations: self.do_mouth_calculations,
            eye_blink_ratio: self.eye_blink_ratio,
            input_image_x: self.input_image_x,
            input_image_y: self.input_image_y,
            image_scale: self.image_scale,
        })
    }
}

pub struct FaceProcessor {
    #[getset(get = "pub")]
    backend_setting: BackendProviders,
    #[getset(get = "pub")]
    backend_held: Box<dyn FaceProcessorTrait>,
    #[getset(get_copy = "pub", set = "pub")]
    do_eye_calculations: bool,
    #[getset(get_copy = "pub", set = "pub")]
    do_mouth_calculations: bool,
    #[getset(get_copy = "pub", set = "pub")]
    eye_blink_ratio: f64,
    #[getset(get_copy = "pub", set = "pub")]
    input_image_x: u32,
    #[getset(get_copy = "pub", set = "pub")]
    input_image_y: u32,
    #[getset(get_copy = "pub", set = "pub")]
    image_scale: Option<ImageScale>,
}

impl FaceProcessor {
    pub fn calculate_raw_landmarks(
        &self,
        image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Vec<FaceLandmark> {
        self.backend_held.get_face_landmark(image)
    }

    pub fn calculate_pnp(&self, image: &ImageBuffer<Rgb<u8>, Vec<u8>>, landmark: FaceLandmark) {}

    pub fn calculate_pnps(
        &self,
        image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        landmark: Vec<FaceLandmark>,
    ) {
    }
}
