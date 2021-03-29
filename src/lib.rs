#[cfg(feature = "opencl")]
extern crate cl3;
#[cfg(feature = "vulkan")]
extern crate vulkano;
#[cfg(feature = "apachetvm")]
extern crate tvm;
#[cfg(feature = "tf")]
extern crate tflite;

mod error;
mod enumerate_devices;
mod face_processor_trait;
mod utils;
mod backends;