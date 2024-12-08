use std::ffi::NulError;
use crate::metal::ErrorMetal;
use crate::vulkan::ErrorVulkan;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "metal")]
    #[error("Metal backend: {0}")]
    MetalBackend(#[from] ErrorMetal),
    #[cfg(feature = "vulkan")]
    #[error("Vulkan backend: {0}")]
    VulkanBackend(#[from] ErrorVulkan),
    #[error("Hassle rs error: {0}")]
    HassleRs(#[from] hassle_rs::HassleError),
    #[error("Nul error: {0}")]
    NulError(#[from] NulError)
}
