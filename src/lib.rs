//!  Decoding and Encoding of WBMP Images
//!  
//!  A decoder and encoder for WBMP (Wireless Bitmap Format) images
//!  
//!  # Related Links
//!  * <https://www.wapforum.org/what/technical/SPEC-WAESpec-19990524.pdf>

pub mod color;
pub mod decoder;
pub mod encoder;
pub mod error;

pub use crate::color::ColorType;
pub use crate::decoder::Decoder;
pub use crate::encoder::Encoder;
