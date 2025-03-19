use crate::metal::DeviceMetal;
use crate::shader::{ShaderModule, ShaderModuleCreateDesc};
use crate::{Error, PhysicalDevice};

pub struct DeviceCreateDesc {
    pub physical_device: PhysicalDevice,
}

pub enum Device {
    #[cfg(feature = "metal")]
    Metal(DeviceMetal),
}

impl Device {
    pub fn create_shader(&self, desc: &ShaderModuleCreateDesc) -> Result<ShaderModule, Error> {
        match self {
            Device::Metal(device) => Ok(ShaderModule::Metal(device.create_shader(desc)?)),
        }
    }
}
