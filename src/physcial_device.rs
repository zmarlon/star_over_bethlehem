use crate::metal::PhysicalDeviceMetal;

#[derive(Clone)]
pub enum PhysicalDevice {
    Metal(PhysicalDeviceMetal),
}

impl PhysicalDevice {
    #[inline]
    pub fn name(&self) -> &str {
        match self {
            #[cfg(feature = "metal")]
            PhysicalDevice::Metal(physical_device) => physical_device.name(),
        }
    }
}
