// TODO: Unified detector that abstracts over a backend here

use crate::face_processor_trait::FaceProcessorTrait;
use crate::utils::misc::{BackendProviders, ImageScale};
use image::imageops::FilterType;
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
    quality: i8,
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
            quality: 3,
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
            quality: self.quality,
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
            quality: self.quality,
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
            quality: self.quality,
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
            quality: self.quality,
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
            quality: self.quality,
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
            quality: self.quality,
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
            quality: self.quality,
        }
    }

    pub fn with_quality(self, quality: i8) -> Self {
        FaceProcessorBuilder {
            backend: self.backend,
            desired_threads: self.desired_threads,
            do_eye_calculations: self.do_eye_calculations,
            do_mouth_calculations: self.do_mouth_calculations,
            eye_blink_ratio: self.eye_blink_ratio,
            input_image_x: self.input_image_x,
            input_image_y: self.input_image_y,
            image_scale: self.image_scale,
            quality,
        }
    }

    pub fn build(self) -> Result<FaceProcessor, ()> {
        Err(())
    }
}

pub struct FaceProcessor {
    backend_setting: Cell<BackendProviders>,
    backend_held: RefCell<Box<dyn FaceProcessorTrait>>,
    do_eye_calculations: Cell<bool>,
    do_mouth_calculations: Cell<bool>,
    eye_blink_ratio: Cell<f64>,
    input_image_x: Cell<u32>,
    input_image_y: Cell<u32>,
    image_scale: Cell<Option<ImageScale>>,
    quality: Cell<i8>,
}

impl FaceProcessor {}
