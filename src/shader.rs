use std::borrow::Cow;
use crate::metal::ShaderModuleMetal;

pub enum ShaderModule {
    #[cfg(feature = "metal")]
    Metal(ShaderModuleMetal)
}

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

pub enum ShaderSource {
    Hlsl {
        source: Cow<'static, str>,
        defines: Vec<(Cow<'static, str>, Option<Cow<'static, str>>)>,
    },
}