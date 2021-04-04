use crate::enumerate_devices::device::{DeviceBackend, DevicePointer};
#[cfg(feature = "opencl")]
use cl3::{
    device::{get_device_ids, get_device_info, DeviceInfo, CL_DEVICE_TYPE_GPU, CL_DEVICE_TYPE_ACCELERATOR, CL_DEVICE_TYPE_CPU, CL_DEVICE_TYPE_CUSTOM, CL_DEVICE_TYPE_ALL, CL_DEVICE_TYPE_DEFAULT},
    info_type::InfoType,
    platform::get_platform_ids,
};
use std::{mem::MaybeUninit, sync::Arc};
#[cfg(feature = "vulkan")]
use vulkano::instance::{Instance, InstanceCreationError, InstanceExtensions, PhysicalDevice};
use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
pub enum OpenCLEnumerateType {
    All = 4294967295,
    Cpu = 1 << 1,
    Custom = 1 << 4,
    Default = 1 << 0,
    Gpu = 1 << 2,
}
impl Into<u64> for OpenCLEnumerateType {
    fn into(self) -> u64 {
        self as u64
    }
}
impl Default for OpenCLEnumerateType {
    fn default() -> Self {
        OpenCLEnumerateType::Gpu
    }
}

/// A way to access system properties. Used to enumerate_devices devices to feed into TVM.
///
/// Note that CPU enumeration is not supported at this time. Just use the default index for the CPU.
pub struct System {
    vk_instance: MaybeUninit<Arc<Instance>>,
}

impl System {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut vk_instance: MaybeUninit<Arc<Instance>> = MaybeUninit::uninit();
        if cfg!(feature = "vulkan") {
            let inst = match Instance::new(None, &InstanceExtensions::none(), None) {
                Ok(i) => i,
                Err(why) => {
                    return Err(Box::new(why));
                }
            };
            vk_instance = MaybeUninit::new(inst);
        }
        Ok(System { vk_instance })
    }

    pub fn enumerate_gpus(&self) -> Result<Vec<DevicePointer>, i32> {
        let mut dev_list: Vec<DevicePointer> = vec![];
        if cfg!(feature = "vulkan") {
            dev_list = self.enumerate_vulkan();
        } else if cfg!(feature = "opencl") {
            dev_list = match self.enumerate_opencl(None) {
                Ok(list) => list,
                Err(why) => return Err(why),
            };
        }
        Ok(dev_list)
    }
    #[cfg(feature = "vulkan")]
    pub fn enumerate_vulkan(&self) -> Vec<DevicePointer> {
        let mut device_list: Vec<DevicePointer> = vec![];
        let instance = unsafe { self.vk_instance.assume_init() };
        let devices = PhysicalDevice::enumerate(&instance);
        for device in devices {
            let dev = DevicePointer {
                name: device.name().to_string(),
                index: device.index(),
                backend: DeviceBackend::GpuVulkan,
                vendor_id: device.pci_vendor_id() as usize,
                device_id: 0,
            };
            device_list.push(dev)
        }
        device_list
    }

    #[cfg(feature = "opencl")]
    pub fn enumerate_opencl<S: Into<u64>>(&self, op: Option<S>) -> Result<Vec<DevicePointer>, i32> {
        // check to ensure its a valid value
        let mut query = CL_DEVICE_TYPE_GPU;
        if let Some(q) = op {
            let read = q.into();
            match read {
                CL_DEVICE_TYPE_ACCELERATOR | CL_DEVICE_TYPE_ALL | CL_DEVICE_TYPE_CPU | CL_DEVICE_TYPE_CUSTOM | CL_DEVICE_TYPE_DEFAULT | CL_DEVICE_TYPE_GPU => {
                    query = read;
                }
                _ => {
                    return Err(-512);
                }
            }
        }

        let mut dev_list: Vec<DevicePointer> = vec![];
        let platform_id = match get_platform_ids() {
            Ok(platform) => platform[0],
            Err(why) => {
                return Err(why);
            }
        };
        let device_ids = match get_device_ids(platform_id, query) {
            Ok(devs) => devs,
            Err(why) => return Err(why),
        };
        for device_id in device_ids {
            let device_info_name = match get_device_info(device_id, DeviceInfo::CL_DEVICE_NAME) {
                Ok(i) => i.to_str().unwrap().into_string().unwrap(),
                Err(why) => {
                    return Err(why);
                }
            };

            let device_info_vendor =
                match get_device_info(device_id, DeviceInfo::CL_DEVICE_VENDOR_ID) {
                    Ok(i) => i.to_size(),
                    Err(why) => {
                        return Err(why);
                    }
                };

            let dev = DevicePointer {
                name: device_info_name,
                index: device_id as usize,
                backend: DeviceBackend::ComputeOpenCL(query as OpenCLEnumerateType),
                vendor_id: device_info_vendor,
                device_id: 0,
            };
            dev_list.push(dev)
        }
        Ok(dev_list)
    }
}
