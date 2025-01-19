use std::error::Error;
use std::{io, fmt};

/// Wbmp error kinds
#[derive(Debug)]
pub enum WbmpError {
    /// An I/O Error occurred while decoding the image.
    IoError(io::Error),
    /// An Unsupported Image Type Identifier was encountered while decoding 
    /// the image.
    UnsupportedType(u8),
    /// Unsupported Extension headers were encountered while decoding the 
    /// image.
    UnsupportedHeaders,
    /// The image does not support the requested operation
    UsageError(String),
    /// Image data provided to the Encoder inconsistent with provided type.
    InvalidImageData,
}

impl fmt::Display for WbmpError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            WbmpError::IoError(ref e) => e.fmt(fmt),
            WbmpError::UsageError(ref f) => write!(
                fmt,
                "The requested operation could not be completed {}",
                f,
            ),
            WbmpError::UnsupportedType(ref f) => write!(
                fmt,
                "The Decoder does not support the image type `{}`",
                f
            ),
            WbmpError::UnsupportedHeaders => write!(
                fmt,
                "The Decoder does not support extension headers",
            ),
            WbmpError::InvalidImageData => write!(
                fmt,
                "The Image data does not match the ColorType",
            ),
        }
    }
}

impl Error for WbmpError {
    fn description(&self) -> &str {
        match *self {
            WbmpError::IoError(..) => "IO error",
            WbmpError::UnsupportedType(..) => "Unsupported Type error",
            WbmpError::UnsupportedHeaders => "Unsupported Headers error",
            WbmpError::UsageError(..) => "Usage error",
            WbmpError::InvalidImageData => "Invalid image data error",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            WbmpError::IoError(ref e) => Some(e),
            WbmpError::UnsupportedType(..) => None,
            WbmpError::UnsupportedHeaders => None,
            WbmpError::UsageError(..) => None,
            WbmpError::InvalidImageData => None,
        }
    }
}

impl From<io::Error> for WbmpError {
    fn from(err: io::Error) -> WbmpError {
        WbmpError::IoError(err)
    }
}

/// Result of an image decoding/encoding process
pub type WbmpResult<T> = Result<T, WbmpError>;
