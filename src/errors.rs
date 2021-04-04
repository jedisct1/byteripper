pub use anyhow::Error;
use std::io;
#[derive(Debug, thiserror::Error)]
pub enum BRError {
    #[error("Internal error: {0}")]
    InternalError(&'static str),
    #[error("Incorrect usage: {0}")]
    UsageError(&'static str),
    #[error("{}", _0)]
    Io(#[from] io::Error),
    #[error("Parse error")]
    ParseError,
    #[error("Unsupported")]
    Unsupported,
}
