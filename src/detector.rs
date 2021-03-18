use crate::enumerate_devices::device::DevicePointer;
use tvm::{Context, DeviceType};

pub struct FacialProcessor {
    tvm_ctx: Context,
    tvm_ctx_dev_type: DeviceType,
    tvm_ctx_dev_id: i32,
    #[cfg(feature = "vulkan", feature = "cl3")]
    tvm_dev_ptr: DevicePointer,
}

impl FacialProcessor {}
