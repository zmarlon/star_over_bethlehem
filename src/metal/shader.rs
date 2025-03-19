use crate::metal::{DeviceMetal, ErrorMetal};
use crate::shader::{ShaderKind, ShaderModuleCreateDesc, ShaderSource};
use crate::Error;
use metal_irconverter::sys;
use objc2::__framework_prelude::{ProtocolObject, Retained};
use objc2::runtime::AnyObject;
use objc2::{msg_send, Encode, Encoding, RefEncode};
use objc2_foundation::NSString;
use objc2_metal::{MTLDevice, MTLFunction, MTLLibrary};
use std::ffi::{c_void, CString};
use std::mem;
use std::ops::Deref;

fn get_ir_shader_stage(stage: ShaderKind) -> sys::IRShaderStage {
    match stage {
        ShaderKind::Vertex => sys::IRShaderStage_IRShaderStageVertex,
        ShaderKind::Task => sys::IRShaderStage_IRShaderStageAmplification,
        ShaderKind::Mesh => sys::IRShaderStage_IRShaderStageMesh,
        ShaderKind::Fragment => sys::IRShaderStage_IRShaderStageFragment,
        ShaderKind::Compute => sys::IRShaderStage_IRShaderStageCompute,
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

pub struct __dispatch_object_s {
    _private: [u8; 0],
}

type dispatch_object_t = *mut __dispatch_object_s;
type dispatch_data_t = dispatch_object_t;
type dispatch_data_destructor_t =
    Option<unsafe extern "C" fn(context: *mut c_void, data: dispatch_data_t)>;

#[repr(transparent)]
pub struct DispatchData(dispatch_data_t);

unsafe impl Encode for DispatchData {
    const ENCODING: Encoding = Encoding::Object;
}

unsafe impl RefEncode for DispatchData {
    const ENCODING_REF: Encoding = Encoding::Char;
}

extern "C" {
    fn dispatch_data_create(
        buffer: *const c_void,
        size: usize,
        destructor: dispatch_data_destructor_t,
        context: *mut c_void,
    ) -> DispatchData;
}

unsafe fn create_library_from_blob(
    device: &Retained<ProtocolObject<dyn MTLDevice>>,
    blob: &[u8],
) -> Option<Retained<ProtocolObject<dyn MTLLibrary>>> {
    let dispatch_data = dispatch_data_create(
        blob.as_ptr() as *const c_void,
        blob.len() as _,
        None,
        std::ptr::null_mut(),
    );

    let mut error: *mut AnyObject = std::ptr::null_mut();

    let lib: *mut AnyObject =
        msg_send![&*device, newLibraryWithData: dispatch_data error: &mut error];

    Retained::from_raw(lib as *mut ProtocolObject<dyn MTLLibrary>)
}

pub struct ShaderModuleMetal {
    function: Retained<ProtocolObject<dyn MTLFunction>>,
}

impl ShaderModuleMetal {
    pub fn new(device: &DeviceMetal, desc: &ShaderModuleCreateDesc) -> Result<Self, Error> {
        let target_profile = get_target(desc.kind);

        match &desc.source {
            ShaderSource::Hlsl { source, defines } => {
                let defines = defines
                    .iter()
                    .map(|(k, v)| (k.deref(), v.as_deref()))
                    .collect::<Vec<_>>();
                let shader = hassle_rs::compile_hlsl(
                    &desc.name,
                    source,
                    &desc.entry_point,
                    target_profile,
                    &[],
                    &defines,
                )?;

                let entry_point_cstr = CString::new(&desc.entry_point as &str)?;

                unsafe {
                    let compiler = sys::IRCompilerCreate();
                    sys::IRCompilerSetEntryPointName(compiler, entry_point_cstr.as_ptr());

                    let dxil = sys::IRObjectCreateFromDXIL(
                        shader.as_ptr(),
                        shader.len(),
                        sys::IRBytecodeOwnership_IRBytecodeOwnershipNone,
                    );
                    let mut error = std::ptr::null_mut();
                    let out_ir = sys::IRCompilerAllocCompileAndLink(
                        compiler,
                        entry_point_cstr.as_ptr(),
                        dxil,
                        &mut error,
                    );

                    if out_ir.is_null() {
                        sys::IRErrorDestroy(error);
                        return Err(Error::MetalBackend(ErrorMetal::Custom(
                            "IRCompilerAllocCompileAndLink failed".into(),
                        )));
                    }

                    let metal_lib = sys::IRMetalLibBinaryCreate();
                    sys::IRObjectGetMetalLibBinary(
                        out_ir,
                        get_ir_shader_stage(desc.kind),
                        metal_lib,
                    );
                    let size = sys::IRMetalLibGetBytecodeSize(metal_lib);
                    let mut bytecode = vec![0; size];
                    sys::IRMetalLibGetBytecode(metal_lib, bytecode.as_mut_ptr());

                    sys::IRMetalLibBinaryDestroy(metal_lib);
                    sys::IRObjectDestroy(dxil);
                    sys::IRObjectDestroy(out_ir);

                    sys::IRCompilerDestroy(compiler);

                    let device = device.physical_device().as_metal();
                    let mtl_device = device.mtl_device();

                    let library = create_library_from_blob(&mtl_device, &bytecode).ok_or(
                        Error::MetalBackend(ErrorMetal::Custom(
                            "Failed to create library from blob".into(),
                        )),
                    )?;

                    let entry_point = NSString::from_str(&desc.entry_point);
                    let function = library.newFunctionWithName(entry_point.deref()).ok_or(
                        Error::MetalBackend(ErrorMetal::Custom("Failed to create function".into())),
                    )?;

                    Ok(Self { function })
                }
            }
        }
    }

    pub fn function(&self) -> &Retained<ProtocolObject<dyn MTLFunction>> {
        &self.function
    }
}
