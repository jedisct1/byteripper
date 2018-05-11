use std::io;

#[derive(Debug, Fail)]
pub enum BRError {
    #[fail(display = "Internal error: {}", _0)]
    InternalError(&'static str),
    #[fail(display = "Incorrect usage: {}", _0)]
    UsageError(&'static str),
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "Parse error")]
    ParseError,
    #[fail(display = "Unsupported")]
    Unsupported,
}

impl From<io::Error> for BRError {
    fn from(e: io::Error) -> BRError {
        BRError::Io(e)
    }
}
