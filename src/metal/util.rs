use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorMetal {
    #[error("{0}")]
    Custom(String),
}
