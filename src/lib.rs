#[cfg(feature = "opencl")]
extern crate cl3;
#[cfg(feature = "vulkan")]
extern crate vulkano;

mod error;
mod enumerate_devices;
mod detector;