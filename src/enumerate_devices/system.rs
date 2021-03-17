use crate::device::{DeviceBackend, DevicePointer};
use cl3::{
    device::{
        get_device_ids,
        get_device_info,
        CL_DEVICE_TYPE_GPU,
        DeviceInfo
    },
    platform::get_platform_ids,
    info_type::InfoType
};
use std::{mem::MaybeUninit, sync::Arc};
#[cfg(feature = "vulkan")]
use vulkano::instance::{Instance, InstanceCreationError, InstanceExtensions, PhysicalDevice};

/// A way to access system properties. Used to enumerate_devices devices to feed into TVM.
///
/// Note that CPU enumeration is not supported at this time. Just use the default index for the CPU.
pub struct System {
    #[cfg(feature = "vulkan")]
    vk_instance: MaybeUninit<Arc<Instance>>,
}

impl System {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut vk_instance: MaybeUninit<Arc<Instance>> = MaybeUninit::uninit();
        if cfg!(feature = "vulkan") {
            let inst = match Instance::new(None, &InstanceExtensions::none(), None) {
                Ok(i) => {}
                Err(why) => {
                    return Err(Box::new(why));
                }
            };
            vk_instance = i;
        }

        Ok(System { vk_instance })
    }

    pub fn enumerate_gpus(&self) -> Result<Vec<DevicePointer>, i32> {
        let mut dev_list: Vec<DevicePointer> = vec![];
        if cfg!(feature = "vulkan") {
            dev_list = self.enumerate_vulkan();
        } else if cfg!(feature = "opencl") {
            dev_list = match self.enumerate_opencl() {
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
    pub fn enumerate_opencl(&self) -> Result<Vec<DevicePointer>, i32> {
        let mut dev_list: Vec<DevicePointer> = vec![];
        let platform_id = match get_platform_ids() {
            Ok(platform) => {
                platform[0]
            }
            Err(why) => {
                return Err(why);
            }
        };
        let device_ids = match get_device_ids(platform_id, CL_DEVICE_TYPE_GPU) {
            Ok(devs) => devs,
            Err(why) => return Err(why),
        };
        for device_id in device_ids {
            let device_info_name = match get_device_info(device_id, DeviceInfo::CL_DEVICE_NAME) {
                Ok(i) => {
                    i.to_str().unwrap().into_string().unwrap()
                }
                Err(why) => {
                    return Err(why);
                }
            };

            let device_info_vendor = match get_device_info(device_id, DeviceInfo::CL_DEVICE_VENDOR_ID) {
                Ok(i) => {
                    i.to_size()
                }
                Err(why) => {
                    return Err(why);
                }
            };

            let dev = DevicePointer {
                name: device_info_name,
                index: device_id as usize,
                backend: DeviceBackend::ComputeOpenCL,
                vendor_id: device_info_vendor,
                device_id: 0,
            };
            dev_list.push(dev)
        }
        Ok(dev_list)
    }
}
