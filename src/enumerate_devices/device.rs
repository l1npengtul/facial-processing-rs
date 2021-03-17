/// Types of devices supported by this crate. Note the exclusions of CPU, since it is expected that there is only 1 per device, they need no enumeration.
///
/// GpuVulkan => GPU that is capable of vulkan. NOTE: Only possible if you enabled the `vulkan` feature.
///
/// GpuOpenGL => A GPU that is capable of OpenGL 3. NOTE: Only possible if you enabled the `opengl` feature.
///
/// ComputeOpenCL => A GPU that is capable of OpenCL 3. NOTE: Only possible if you enabled the `opencl` feature.
#[derive(Copy, Clone, Debug)]
pub enum DeviceBackend {
    GpuVulkan,
    GpuOpenGL,
    ComputeOpenCL,
}

/// A struct that holds basic information of a device, enough for it to be shown to a user and accessed by the TVM Runtime.
#[derive(Clone, Debug)]
pub struct DevicePointer {
    pub name: String,
    pub index: usize,
    pub backend: DeviceBackend,
    pub vendor_id: usize,
    pub device_id: usize,
}
