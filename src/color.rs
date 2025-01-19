
/// Color type of the image.
///
/// Wbmp does not support color images. This enum is used to indicate the
/// format of image data provided to the encoder, and desired output format
/// of the decoder (as yet unsupported).
pub enum ColorType {
    /// Image with red, green, blue and alpha byte per pixel.
    Rgba8,
    /// Image with a single byte luminance per pixel.
    Luma8,
}
