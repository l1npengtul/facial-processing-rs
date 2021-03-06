// TODO: Unified detector that abstracts over a backend here

#[cfg(feature = "dlib")]
use crate::backends::dlib::dlib_processor::DLibProcessor;
#[cfg(feature = "openvtuber")]
use crate::backends::openvtuber::openvt_processor::OpenVTFaceProcessor;
// Do not remove the `PnPArguments, Point2D` imports. They are cursed. It will not compile on downstream users otherwise. 
use crate::{error::FacialProcessingError, face_processor_trait::FaceProcessorTrait, utils::{eyes::Eye, face::FaceLandmark, misc::{BackendProviders, BoundingBox, EulerAngles, ImageScale, LeftRight, PnPSolver, PnPArguments, Point2D}}};
use image::{imageops::FilterType, ImageBuffer, Rgb};

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
            backend: BackendProviders::None,
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

    #[cfg(feature = "dlib")]
    pub fn build(self) -> Result<FaceProcessor, FacialProcessingError> {
        let backend_held: Box<dyn FaceProcessorTrait> = match self.backend.clone() {
            // BackendProviders::OpenVTuber {
            //     face_detector_path,
            //     face_alignment_path,
            //     face_eyesolator_path,
            // } => {
            //     match OpenVTFaceProcessor::new(
            //         face_detector_path,
            //         face_alignment_path,
            //         face_eyesolator_path,
            //     ) {
            //         Ok(process) => Box::new(process),
            //         Err(why) => return Err(why),
            //     }
            // }
            // TODO: Fix
            BackendProviders::DLib {
                face_alignment_path,
            } => match DLibProcessor::new(face_alignment_path) {
                Ok(process) => Box::new(process),
                Err(why) => return Err(why),
            },
            _ => {
                return Err(FacialProcessingError::InitializeError(
                    "unsupported!".to_string(),
                ))
            }
        };

        let pnp = PnPSolver::new(
            Point2D::new(self.input_image_x as f64, self.input_image_y as f64),
            None,
            PnPArguments::NoRandsc,
        )
        .unwrap();

        Ok(FaceProcessor {
            backend_setting: self.backend,
            backend_held,
            do_eye_calculations: self.do_eye_calculations,
            do_mouth_calculations: self.do_mouth_calculations,
            eye_blink_ratio: self.eye_blink_ratio,
            input_image_x: self.input_image_x,
            input_image_y: self.input_image_y,
            image_scale: self.image_scale,
            pnp,
        })
    }
}

impl Default for FaceProcessorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FaceProcessor {
    backend_setting: BackendProviders,
    backend_held: Box<dyn FaceProcessorTrait>,
    do_eye_calculations: bool,
    do_mouth_calculations: bool,
    eye_blink_ratio: f64,
    input_image_x: u32,
    input_image_y: u32,
    image_scale: Option<ImageScale>,
    pnp: PnPSolver,
}

impl FaceProcessor {
    pub fn calculate_face_bboxes(&self, image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<BoundingBox> {
        self.backend_held.get_face_detections(image)
    }

    pub fn calculate_landmark(
        &self,
        image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        bbox: BoundingBox,
    ) -> FaceLandmark {
        self.backend_held.get_face_landmark(image, bbox)
    }

    pub fn calculate_pnp(
        &self,
        _image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
        landmark: FaceLandmark,
    ) -> Result<EulerAngles, FacialProcessingError> {
        self.pnp.forward(landmark)
    }

    pub fn calculate_eyes(
        &self,
        landmark: FaceLandmark,
        image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> [Eye; 2] {
        let e1 = Eye::new(&landmark, LeftRight::Left, image);
        let e2 = Eye::new(&landmark, LeftRight::Right, image);
        [e1, e2]
    }

    /// Get a reference to the face processor's backend setting.
    pub fn backend_setting(&self) -> &BackendProviders {
        &self.backend_setting
    }

    /// Get a reference to the face processor's do eye calculations.
    pub fn do_eye_calculations(&self) -> &bool {
        &self.do_eye_calculations
    }

    /// Get a reference to the face processor's do mouth calculations.
    pub fn do_mouth_calculations(&self) -> &bool {
        &self.do_mouth_calculations
    }

    /// Get a reference to the face processor's eye blink ratio.
    pub fn eye_blink_ratio(&self) -> &f64 {
        &self.eye_blink_ratio
    }

    /// Set the face processor's eye blink ratio.
    pub fn set_eye_blink_ratio(&mut self, eye_blink_ratio: f64) {
        self.eye_blink_ratio = eye_blink_ratio;
    }

    /// Get a reference to the face processor's input image x.
    pub fn input_image_x(&self) -> &u32 {
        &self.input_image_x
    }

    /// Get a reference to the face processor's input image y.
    pub fn input_image_y(&self) -> &u32 {
        &self.input_image_y
    }

    /// Get a reference to the face processor's image scale.
    pub fn image_scale(&self) -> &Option<ImageScale> {
        &self.image_scale
    }

    /// Set the face processor's image scale.
    pub fn set_image_scale(&mut self, image_scale: Option<ImageScale>) {
        self.image_scale = image_scale;
    }
}
