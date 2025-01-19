
use std::io::Write;

use crate::color::ColorType;
use crate::error::{
    WbmpResult, WbmpError,
};

/// Default threshold value for `Encoder`
const DEFAULT_THRESHOLD: u8 = 0x7F;

/// A WBMP encoder
pub struct Encoder<'a, W: 'a> {
    writer: &'a mut W,
    threshold: u8,
}

impl<'a, W: Write> Encoder<'a, W> {

    /// Create a new decoder that decodes from the stream ```reader```.
    ///
    /// ## Examples
    /// ```
    /// use std::fs::File;
    /// use wbmp::Encoder;
    /// let img_data = vec![
    ///     0xFF, 0x00, 0xFF, 0x00,
    ///     0xFF, 0x00, 0xFF, 0x00,
    ///     0xFF, 0x00, 0xFF, 0x00,
    ///     0xFF, 0x00, 0xFF, 0x00,
    /// ];
    /// let (width, height) = (4, 4);
    /// let mut out_file = vec![];
    /// let mut encoder = Encoder::new(&mut out_file);
    /// encoder.encode(
    ///     &img_data, width, height, wbmp::ColorType::Luma8
    /// ).unwrap()
    /// ```
    pub fn new(writer: &'a mut W) -> Self {
        Encoder {
            writer,
            threshold: DEFAULT_THRESHOLD,
        }
    }

    /// Set the greyscale threshold to use.
    ///
    /// Greyscale values below this threshold (inclusive) will result in 
    /// black pixels while values above will be white. Default 127.
    pub fn with_threshold(mut self, threshold: u8) -> Self {
        self.threshold = threshold;

        self
    }

    fn bytes_from_u32(a: u32) -> Vec<u8> {
        // break it up into 7 bit chunks
        const BIT_MASK: u32 = 0b1111111;
        let mut result = vec![(a & BIT_MASK) as u8];
        let mut a = a >> 7;
        while a > 0 {
            result.insert(0, 0b10000000 | ((a & BIT_MASK) as u8));
            a = a >> 7;
        }
        result
    }

    /// Encodes the image `image` with dimensions `width` by `height` and 
    /// `ColorType` `color_type`.
    pub fn encode(
        &mut self, 
        image: &[u8], 
        width: u32, 
        height: u32, 
        color_type: ColorType,
    ) -> WbmpResult<()> {
        // write headers
        let type_fix_header_fields: &[u8; 2] = &[0_u8; 2];
        let width_bytes = Self::bytes_from_u32(width);
        let height_bytes = Self::bytes_from_u32(height);

        self.writer.write(type_fix_header_fields)?;
        self.writer.write(&width_bytes)?;
        self.writer.write(&height_bytes)?;

        // map and write image data
        match color_type {
            ColorType::Rgba8 => self.encode_rgba8(image, width, height)?,
            ColorType::Luma8 => self.encode_luma8(image, width, height)?,
        }
        Ok(())
    }

    fn encode_luma8(
        &mut self,
        image: &[u8],
        width: u32,
        height: u32,
    ) -> WbmpResult<()> {
        let data_len = (width * height) as usize;
        if data_len != image.len() {
            return Err(WbmpError::InvalidImageData);
        }

        for row in image.chunks(width as usize) {
            let mut byte: u8 = 0;
            let mut bits: u8 = 0;
            for i in 0..row.len() {

                let pixel = row[i];

                if pixel >= self.threshold {
                    byte |= 1<<(7-bits);
                }

                bits += 1;

                if bits == 8 || i == width as usize - 1 {
                    self.writer.write(&[byte; 1])?;
                    byte = 0;
                    bits = 0;
                }
            }
        }
        self.writer.flush()?;

        Ok(())
    }

    fn encode_rgba8(
        &mut self,
        image: &[u8],
        width: u32,
        height: u32,
    ) -> WbmpResult<()> {
        let data_len = (width * height * 4) as usize;
        if data_len != image.len() {
            return Err(WbmpError::InvalidImageData);
        }

        // average components & map using threshold
        for row in image.chunks(width as usize * 4) {
            let mut byte: u8 = 0;
            let mut bits: u8 = 0;
            for (i, pixel) in row.chunks(4).enumerate() {

                let pixel = pixel.iter()
                    .map(|c| *c as usize)
                    .sum::<usize>() / 4;

                if pixel >= self.threshold as usize {
                    byte |= 1<<(7-bits);
                }

                bits += 1;

                if bits == 8 || i == width as usize - 1 {
                    self.writer.write(&[byte; 1])?;
                    byte = 0;
                    bits = 0;
                }
            }
        }
        self.writer.flush()?;

        Ok(())
    }

}


