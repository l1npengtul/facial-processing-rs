use thiserror::Error;

#[derive(Error, Debug)]
pub enum FacialProcessingError {
    #[error("Could not enumerate_devices devices: {0}")]
    EnumerateDeviceError(String),
    #[error("Cannot Find File: {0}")]
    CannotFindFileError(String),
    #[error("Cannot Initialize Facial Processor: {0}")]
    InitializeError(String)
}