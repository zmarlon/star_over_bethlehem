use std::sync::Arc;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::MTLDevice;

use crate::Error;

struct Inner {
    mtl_device: Retained<ProtocolObject<dyn MTLDevice>>,
    name: String,
}

#[derive(Clone)]
pub struct PhysicalDeviceMetal(Arc<Inner>);

impl PhysicalDeviceMetal {
    pub fn new(mtl_device: Retained<ProtocolObject<dyn MTLDevice>>) -> Result<Self, Error> {
        let name = mtl_device.name().to_string();

        Ok(Self(Arc::new(Inner { mtl_device, name })))
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn mtl_device(&self) -> Retained<ProtocolObject<dyn MTLDevice>> {
        self.0.mtl_device.clone()
    }
}
