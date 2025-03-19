use crate::metal::{GraphicsPipelineMetal, PhysicalDeviceMetal};
use crate::{PhysicalDevice, ShaderModule};
use std::hint;

pub struct GraphicsPipelineDesc {
    pub shader_stages: Vec<ShaderModule>,
}

pub enum GraphicsPipeline {
    Metal(GraphicsPipelineMetal),
}

impl GraphicsPipeline {
    #[inline]
    pub unsafe fn as_metal(&self) -> &GraphicsPipelineMetal {
        match self {
            GraphicsPipeline::Metal(pipeline) => pipeline,
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }
}
