use objc2::rc::Retained;
use objc2_foundation::NSError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorMetal {
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    NsError(#[from] Retained<NSError>),
}
