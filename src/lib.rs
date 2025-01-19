//!  Decoding and Encoding of WBMP Images
//!  
//!  A decoder and encoder for WBMP (Wireless Bitmap Format) images
//!  
//!  # Related Links
//!  * <https://www.wapforum.org/what/technical/SPEC-WAESpec-19990524.pdf>

pub mod decoder;
pub mod encoder;
mod error;
pub mod color;

pub use self::error::WbmpError;
