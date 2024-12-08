use std::ffi::CString;
use std::ops::Deref;
use metal_irconverter::sys;
use crate::Error;
use crate::shader::{ShaderKind, ShaderModuleCreateDesc, ShaderSource};

fn get_ir_shader_stage(stage: ShaderKind) -> sys::IRShaderStage {
    match stage {
        ShaderKind::Vertex => sys::IRShaderStage_IRShaderStageVertex,
        ShaderKind::Task => sys::IRShaderStage_IRShaderStageAmplification,
        ShaderKind::Mesh => sys::IRShaderStage_IRShaderStageMesh,
        ShaderKind::Fragment => sys::IRShaderStage_IRShaderStageFragment,
        ShaderKind::Compute => sys::IRShaderStage_IRShaderStageCompute
    }
}

fn get_target(stage: ShaderKind) -> &'static str {
    match stage {
        ShaderKind::Vertex => "vs_6_7",
        ShaderKind::Task => "as_6_7",
        ShaderKind::Mesh => "ms_6_7",
        ShaderKind::Fragment => "ps_6_7",
        ShaderKind::Compute => "cs_6_7",
    }
}

pub struct ShaderModuleMetal {
    bytecode: Vec<u8>
}

impl ShaderModuleMetal {
    pub fn new(desc: &ShaderModuleCreateDesc) -> Result<Self, Error> {
        let target_profile = get_target(desc.kind);

        match &desc.source {
            ShaderSource::Hlsl { source, defines } => {
                let defines = defines.iter().map(|(k, v)| (k.deref(), v.as_deref())).collect::<Vec<_>>();
                let shader = hassle_rs::compile_hlsl(&desc.name, source, &desc.entry_point, target_profile,
                                                     &[], &defines)?;

                let entry_point_cstr = CString::new(&desc.entry_point as &str)?;

                unsafe {
                    let compiler = sys::IRCompilerCreate();
                    sys::IRCompilerSetEntryPointName(compiler, entry_point_cstr.as_ptr());

                    let dxil = sys::IRObjectCreateFromDXIL(shader.as_ptr(), shader.len(), sys::IRBytecodeOwnership_IRBytecodeOwnershipNone);
                    let mut error = std::ptr::null_mut();
                    let out_ir = sys::IRCompilerAllocCompileAndLink(compiler, entry_point_cstr.as_ptr(),
                        dxil, &mut error);

                    if out_ir.is_null() {
                        sys::IRErrorDestroy(error);
                    }

                    let metal_lib = sys::IRMetalLibBinaryCreate();
                    sys::IRObjectGetMetalLibBinary(out_ir, get_ir_shader_stage(desc.kind), metal_lib);
                    let size = sys::IRMetalLibGetBytecodeSize(metal_lib);
                    let mut bytecode = vec![0; size];
                    sys::IRMetalLibGetBytecode(metal_lib, bytecode.as_mut_ptr());

                    sys::IRMetalLibBinaryDestroy(metal_lib);
                    sys::IRObjectDestroy(dxil);
                    sys::IRObjectDestroy(out_ir);

                    sys::IRCompilerDestroy(compiler);

                    Ok(ShaderModuleMetal {
                        bytecode
                    })
                }
            }
        }
    }
}
