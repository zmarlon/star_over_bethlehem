use crate::metal::{GraphicsPipelineMetal, ShaderModuleMetal};
use crate::GraphicsPipeline;
use std::borrow::Cow;
use std::hint;

pub struct ShaderModule {
    pub(crate) inner: ShaderModuleInner,
    pub(crate) desc: ShaderModuleCreateDesc,
}

impl ShaderModule {
    pub fn kind(&self) -> ShaderKind {
        self.desc.kind
    }

    pub fn name(&self) -> &str {
        &self.desc.name
    }
}

pub enum ShaderModuleInner {
    #[cfg(feature = "metal")]
    Metal(ShaderModuleMetal),
}

#[derive(Clone)]
pub struct ShaderModuleCreateDesc {
    pub name: Cow<'static, str>,
    pub source: ShaderSource,
    pub kind: ShaderKind,
    pub entry_point: Cow<'static, str>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShaderKind {
    Vertex,
    Task,
    Mesh,
    Fragment,
    Compute,
}

#[derive(Clone)]
pub enum ShaderSource {
    Hlsl {
        source: Cow<'static, str>,
        defines: Vec<(Cow<'static, str>, Option<Cow<'static, str>>)>,
    },
}

impl ShaderModule {
    #[inline]
    pub unsafe fn as_metal(&self) -> &ShaderModuleMetal {
        match &self.inner {
            ShaderModuleInner::Metal(shader) => shader,
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }
}
