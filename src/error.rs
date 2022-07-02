use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::{error, fmt};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    FloatParse,
    IntParse,
    InvalidFormat,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::FloatParse => write!(f, "failed to parse float"),
            Error::IntParse => write!(f, "failed to parse integer"),
            Error::InvalidFormat => write!(f, "invalid format"),
        }
    }
}

impl error::Error for Error {}

impl From<ParseFloatError> for Error {
    fn from(_error: ParseFloatError) -> Self {
        Self::FloatParse
    }
}

impl From<ParseIntError> for Error {
    fn from(_error: ParseIntError) -> Self {
        Self::IntParse
    }
}
