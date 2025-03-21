use crate::metal::{GraphicsPipelineMetal, ShaderModuleMetal};
use crate::{
    DeviceCreateDesc, Error, GraphicsPipelineDesc, PhysicalDevice, ShaderModule,
    ShaderModuleCreateDesc,
};
use std::sync::Arc;

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
        ShaderModuleMetal::new(self, desc)
    }

    pub fn create_graphics_pipeline(
        &self,
        desc: &GraphicsPipelineDesc,
    ) -> Result<GraphicsPipelineMetal, Error> {
        Ok(GraphicsPipelineMetal::new(self, desc)?)
    }

    pub fn physical_device(&self) -> &PhysicalDevice {
        &self.0.physical_device
    }
}
