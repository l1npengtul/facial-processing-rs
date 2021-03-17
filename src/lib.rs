#[cfg(feature = "opencl")]
extern crate cl3;
#[cfg(feature = "vulkan")]
extern crate vulkano;

mod device;
mod error;
mod system;
