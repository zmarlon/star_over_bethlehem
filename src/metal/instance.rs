use crate::metal::util::ErrorMetal;
use crate::metal::{DeviceMetal, PhysicalDeviceMetal};
use crate::{DeviceCreateDesc, Error, Instance, InstanceCreateDesc, PhysicalDevice};
use objc2::rc::Retained;
use objc2_metal::MTLCopyAllDevices;
use std::sync::Arc;

struct Inner {
    physical_devices: Vec<PhysicalDevice>,
    desc: InstanceCreateDesc,
}

#[derive(Clone)]
pub struct InstanceMetal(Arc<Inner>);

impl InstanceMetal {
    pub fn new(instance_desc: &InstanceCreateDesc) -> Result<Self, Error> {
        let devices = {
            let ptr = unsafe { MTLCopyAllDevices().as_ptr() };
            unsafe { Retained::retain(ptr) }.ok_or(Error::MetalBackend(ErrorMetal::Custom(
                String::from("Failed to get metal devices"),
            )))
        }?;

        let physical_devices = devices
            .into_iter()
            .map(|device| Ok(PhysicalDevice::Metal(PhysicalDeviceMetal::new(device)?)))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Self(Arc::new(Inner {
            physical_devices,
            desc: instance_desc.clone(),
        })))
    }

    pub fn create_device(
        &self,
        device_create_desc: &DeviceCreateDesc,
    ) -> Result<DeviceMetal, Error> {
        Ok(DeviceMetal::new(device_create_desc)?)
    }

    #[inline]
    pub fn get_physical_devices(&self) -> &[PhysicalDevice] {
        &self.0.physical_devices
    }

    #[inline]
    pub fn desc(&self) -> &InstanceCreateDesc {
        &self.0.desc
    }
}
