use tvm::{Context, DeviceType};

pub struct FacialProcessor {
    tvm_ctx: Context,
    tvm_ctx_dev_type: DeviceType,
    tvm_ctx_dev_id: i32,

}