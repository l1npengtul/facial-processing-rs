use thiserror::Error;

#[derive(Error, Debug)]
pub enum FacialProcessingError {
    #[error("Could not enumerate devices: {0}")]
    EnumerateDeviceError(String)
}