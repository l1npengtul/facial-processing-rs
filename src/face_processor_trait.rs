use std::ops::Deref;
use std::path::Path;
use crate::error::FacialProcessingError;
use crate::utils::{SupportedProcesses, BoundingBox};

pub trait FaceProcessor {
    fn init() -> Result<(), FacialProcessingError>;
    fn get_supported_processes() -> Vec<SupportedProcesses>;
    fn get_detections() -> Vec<BoundingBox<u16>>;
}
