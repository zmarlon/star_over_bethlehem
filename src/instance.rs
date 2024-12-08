use crate::metal::InstanceMetal;
use crate::{Device, DeviceCreateDesc, Error, PhysicalDevice};

pub enum Instance {
    #[cfg(feature = "metal")]
    Metal(InstanceMetal),
}

#[derive(Clone)]
pub struct InstanceCreateDesc {
    pub api_validation: bool,
    pub backend_type: BackendType,
}

impl Instance {
    pub fn new(desc: &InstanceCreateDesc) -> Result<Self, Error> {
        match desc.backend_type {
            #[cfg(feature = "metal")]
            BackendType::Metal => Ok(Self::Metal(InstanceMetal::new(desc)?)),
            #[cfg(feature = "vulkan")]
            BackendType::Vulkan => {
                todo!()
            }
        }
    }

    pub fn create_device(&self, device_create_desc: &DeviceCreateDesc) -> Result<Device, Error> {
        match self {
            #[cfg(feature = "metal")]
            Instance::Metal(instance) => {
                Ok(Device::Metal(instance.create_device(device_create_desc)?))
            }
        }
    }

    pub fn desc(&self) -> &InstanceCreateDesc {
        match self {
            #[cfg(feature = "metal")]
            Instance::Metal(instance) => instance.desc(),
        }
    }

    #[inline]
    pub fn get_physical_devices(&self) -> &[PhysicalDevice] {
        match self {
            #[cfg(feature = "metal")]
            Instance::Metal(instance) => instance.get_physical_devices(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BackendType {
    #[cfg(feature = "metal")]
    Metal,
    #[cfg(feature = "vulkan")]
    Vulkan,
}
