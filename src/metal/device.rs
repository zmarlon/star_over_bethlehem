use crate::{DeviceCreateDesc, Error, PhysicalDevice, ShaderModule, ShaderModuleCreateDesc};
use std::sync::Arc;
use crate::metal::ShaderModuleMetal;

struct Inner {
    physical_device: PhysicalDevice,
}

#[derive(Clone)]
pub struct DeviceMetal(Arc<Inner>);

impl DeviceMetal {
    pub fn new(device_create_desc: &DeviceCreateDesc) -> Result<Self, Error> {
        Ok(DeviceMetal(Arc::new(Inner {
            physical_device: device_create_desc.physical_device.clone(),
        })))
    }

    pub fn create_shader(&self, desc: &ShaderModuleCreateDesc) -> Result<ShaderModuleMetal, Error> {
        ShaderModuleMetal::new(desc)
    }
}
