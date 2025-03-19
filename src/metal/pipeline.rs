use crate::metal::{DeviceMetal, ErrorMetal, ShaderModuleMetal};
use crate::{GraphicsPipelineDesc, ShaderKind};
use objc2::__framework_prelude::{ProtocolObject, Retained};
use objc2_metal::{
    MTLDevice, MTLMeshRenderPipelineDescriptor, MTLPipelineOption, MTLRenderPipelineDescriptor,
    MTLRenderPipelineState,
};

pub struct GraphicsPipelineMetal {
    pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
}

impl GraphicsPipelineMetal {
    fn is_mesh_pipeline(desc: &GraphicsPipelineDesc) -> bool {
        desc.shader_stages
            .iter()
            .find(|p| p.kind() == ShaderKind::Mesh)
            .is_some()
    }
    fn function(desc: &GraphicsPipelineDesc, kind: ShaderKind) -> Option<&ShaderModuleMetal> {
        desc.shader_stages
            .iter()
            .find(|p| p.kind() == kind)
            .map(|p| unsafe { p.as_metal() })
    }

    pub fn new(device: &DeviceMetal, desc: &GraphicsPipelineDesc) -> Result<Self, ErrorMetal> {
        let mtl_device = unsafe { device.physical_device().as_metal().mtl_device() };

        let fragment_shader = Self::function(desc, ShaderKind::Fragment)
            .ok_or(ErrorMetal::Custom("No fragment shader found".into()))?;

        if Self::is_mesh_pipeline(desc) {
            let mut descriptor = unsafe { MTLMeshRenderPipelineDescriptor::new() };

            if let Some(object_shader) = Self::function(desc, ShaderKind::Task) {
                unsafe { descriptor.setObjectFunction(Some(object_shader.function())) };
            }

            let mesh_shader = Self::function(desc, ShaderKind::Mesh)
                .ok_or(ErrorMetal::Custom("No mesh shader found".into()))?;

            unsafe { descriptor.setMeshFunction(Some(mesh_shader.function())) };
            unsafe { descriptor.setFragmentFunction(Some(fragment_shader.function())) };

            let pipeline = unsafe {
                mtl_device.newRenderPipelineStateWithMeshDescriptor_options_reflection_error(
                    &descriptor,
                    MTLPipelineOption::None,
                    None,
                )?
            };
            Ok(Self { pipeline })
        } else {
            let mut descriptor = MTLRenderPipelineDescriptor::new();

            let vertex_shader = Self::function(desc, ShaderKind::Vertex)
                .ok_or(ErrorMetal::Custom("No vertex shader found".into()))?;

            descriptor.setVertexFunction(Some(vertex_shader.function()));
            descriptor.setFragmentFunction(Some(fragment_shader.function()));

            let pipeline = mtl_device.newRenderPipelineStateWithDescriptor_error(&descriptor)?;

            Ok(Self { pipeline })
        }
    }

    pub fn mtl_pipeline_state(&self) -> &Retained<ProtocolObject<dyn MTLRenderPipelineState>> {
        &self.pipeline
    }
}
