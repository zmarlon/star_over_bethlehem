use crate::metal::PhysicalDeviceMetal;
use std::hint;

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

    #[inline]
    pub unsafe fn as_metal(&self) -> &PhysicalDeviceMetal {
        match self {
            PhysicalDevice::Metal(device) => device,
            _ => unsafe { hint::unreachable_unchecked() },
        }
    }
}
