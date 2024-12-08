use std::str::Utf8Error;

use ash::vk;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorVulkan {
    #[error("{0}")]
    Custom(String),
    #[error("Error: {0}")]
    Error(#[from] vk::Result),
    #[error("{0}")]
    InvalidUtf8(#[from] Utf8Error),
}
