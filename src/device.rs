use crate::metal::DeviceMetal;
use crate::pipeline::{GraphicsPipeline, GraphicsPipelineDesc};
use crate::shader::{ShaderModule, ShaderModuleCreateDesc};
use crate::{Error, PhysicalDevice, ShaderModuleInner};

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
            Device::Metal(device) => Ok(ShaderModule {
                inner: ShaderModuleInner::Metal(device.create_shader(desc)?),
                desc: desc.clone(),
            }),
        }
    }

    pub fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipeline, Error> {
        match self {
            Device::Metal(device) => Ok(GraphicsPipeline::Metal(
                device.create_graphics_pipeline(desc)?,
            )),
        }
    }
}
