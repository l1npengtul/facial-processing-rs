use thiserror::Error;

#[derive(Error, Debug)]
pub enum FacialProcessingError {
    #[error("Could not enumerate_devices devices: {0}")]
    EnumerateDeviceError(String),
    #[error("Error while trying to read file: {0}")]
    IoError(String),
    #[error("Cannot Initialize Facial Processor: {0}")]
    InitializeError(String),
    #[error("Internal Error: {0}")]
    InternalError(String),
}
