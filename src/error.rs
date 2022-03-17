use std::error::Error as StdError;
use std::fmt;

use crate::lz77::LzError;
use crate::mapping::InvalidBlock;

pub type Result<T> = std::result::Result<T, Error>;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    SeekToNullError,
    OutOfBoundsError(usize),
    InvalidAddress(usize, u32),
    LzError(LzError),
    InvalidBlock(InvalidBlock),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::SeekToNullError => None,
            Error::OutOfBoundsError(_) => None,
            Error::InvalidAddress(_, _) => None,
            Error::LzError(err) => Some(err),
            Error::InvalidBlock(err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SeekToNullError => {
                write!(f, "Cannot seek to null address!")
            }
            Error::OutOfBoundsError(address) => {
                write!(
                    f,
                    "Cannot access address at {:#08x} (out of bounds)!",
                    address
                )
            }
            Error::InvalidAddress(address, value) => {
                let bytes = value.to_le_bytes();
                write!(
                    f,
                    "Cannot read \"{:02x} {:02x} {:02x} {:02x}\" at {:#08x} as address!",
                    bytes[0], bytes[1], bytes[2], bytes[3], address
                )
            }
            Error::LzError(err) => err.fmt(f),
            Error::InvalidBlock(err) => err.fmt(f),
        }
    }
}

impl From<LzError> for Error {
    fn from(err: LzError) -> Self {
        Error::LzError(err)
    }
}

impl From<InvalidBlock> for Error {
    fn from(err: InvalidBlock) -> Self {
        Error::InvalidBlock(err)
    }
}
