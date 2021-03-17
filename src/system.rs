use crate::device::{DeviceBackend, DevicePointer};
#[cfg(feature = "cl3")]
use cl3::{
    device::{get_device_ids, get_device_info, CL_DEVICE_TYPE_GPU},
    platform::get_platform_ids,
};
use std::{mem::MaybeUninit, sync::Arc};
#[cfg(feature = "vulkan")]
use vulkano::instance::{Instance, InstanceCreationError, InstanceExtensions, PhysicalDevice};

/// A way to access system properties. Used to enumerate devices to feed into TVM.
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

    pub fn enumerate_gpus(&self) -> Result<Vec<DevicePointer>, Box<dyn std::error::Error>> {
        let mut dev_list: Vec<DevicePointer> = vec![];
        if cfg!(feature = "vulkan") {
            dev_list = self.enumerate_vulkan();
        } else if cfg!(feature = "opencl") {
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

    #[cfg(feature = "cl3")]
    pub fn enumerate_opencl(&self) -> Result<Vec<DevicePointer>, i32> {
        let platform_id = match get_platform_ids() {
            Ok(platform) => {
                // if you don't have a platform id you probably shouldn't be running this software anyway
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
        for device_id in device_ids {}
        Err(0)
    }
}
