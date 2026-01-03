use std::io::Read;

use crate::error::{WbmpError, WbmpResult};

const CONTINUATION_MASK: u8 = 0b10000000;
const DATA_MASK: u8 = 0b01111111;
const EXT_HEADER_TYPE_MASK: u8 = 0b01100000;

/// A WBMP decoder
pub struct Decoder<R> {
    reader: R,
    width: u32,
    height: u32,
}

impl<R: Read> Decoder<R> {
    /// Create a new decoder that decodes from the stream `reader`.
    ///
    /// # Errors
    ///  - `WbmpError::UnsupportedType` occurs if the TypeField in the image
    ///    headers is not set to 0.
    ///  - `WbmpError::UnsupportedHeaders` occurs if extension headers are
    ///    specified in the image. Extension headers are not required for type
    ///    0 WBMP images.
    ///  - `WbmpError::IoError` occurs if the image headers are malformed or
    ///    if another IoError occurs while reading from the provided `reader`
    ///
    /// ## Examples
    /// ```
    /// use wbmp::Decoder;
    /// use std::{fs::File, io::BufReader};
    /// let f = BufReader::new(
    ///     File::open("./tests/images/sample_640x426.wbmp").unwrap()
    /// );
    /// let mut decoder = Decoder::new(f).unwrap();
    /// ```
    pub fn new(reader: R) -> WbmpResult<Decoder<R>> {
        let mut decoder = Self::new_decoder(reader);
        decoder.read_metadata()?;
        Ok(decoder)
    }

    fn new_decoder(reader: R) -> Decoder<R> {
        Decoder {
            reader,
            width: 0,
            height: 0,
        }
    }

    /// Returns the `(width, height)` of the image.
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn read_metadata(&mut self) -> WbmpResult<()> {
        // TypeField
        let image_type_buf: &mut [u8; 1] = &mut [0; 1];
        self.reader.read_exact(image_type_buf)?;
        let image_type = image_type_buf[0];
        // we only support the 00 type, there should never
        // be two bytes to this int
        if image_type != 0 {
            return Err(WbmpError::UnsupportedType(image_type));
        }

        // FixHeaderField
        let fix_header_buf: &mut [u8; 1] = &mut [0; 1];
        self.reader.read_exact(fix_header_buf)?;
        let fix_header = fix_header_buf[0];
        // Bit    Desc
        // 7      Ext Headers flag, 1 = More will follow, 0 = Last octet
        // 6      Ext Header Type, Msb
        // 5      Ext Header Type, Lsb
        // All other bits reserved
        let ext_headers_flag = (fix_header & CONTINUATION_MASK) >> 7;
        let _ext_headers_type = (fix_header & EXT_HEADER_TYPE_MASK) >> 5;

        if ext_headers_flag != 0 {
            return Err(WbmpError::UnsupportedHeaders);
        }

        // Width
        let width_buf: &mut [u8; 1] = &mut [CONTINUATION_MASK; 1];
        // if the continuation bit is set, shift & read the next byte as well
        loop {
            self.reader.read_exact(width_buf)?;
            self.width = (self.width << 7) | (width_buf[0] & DATA_MASK) as u32;
            if width_buf[0] & CONTINUATION_MASK != CONTINUATION_MASK {
                break;
            }
        }

        // Height
        let height_buf: &mut [u8; 1] = &mut [CONTINUATION_MASK; 1];
        // if the continuation bit is set, shift & read the next byte as well
        loop {
            self.reader.read_exact(height_buf)?;
            self.height = (self.height << 7) | (height_buf[0] & DATA_MASK) as u32;
            if height_buf[0] & CONTINUATION_MASK != CONTINUATION_MASK {
                break;
            }
        }

        Ok(())
    }

    /// Reads the image data bits into the provided buffer.
    /// Output data will be `width * height` bytes, 8 bits per pixel.
    pub fn read_image_data(&mut self, buf: &mut [u8]) -> WbmpResult<()> {
        // convert each row, ignoring padding past self.width
        let data_len = (self.width * self.height) as usize;
        if buf.len() < data_len {
            return Err(WbmpError::UsageError(String::from(
                "Provided buffer does not have enough capacity",
            )));
        }

        let row_bytes = if self.width % 8 == 0 {
            self.width / 8
        } else {
            (self.width / 8) + 1
        } as usize;

        let mut bit_data = vec![0_u8; row_bytes];

        let mut read_idx = 0;
        let mut row_bits_read = 0;
        for _ in 0..self.height as usize {
            self.reader.read_exact(&mut bit_data)?;

            'bytes: for byte in bit_data.iter() {
                let mut s = 7;
                'bits: while read_idx < data_len {
                    if byte & (1 << s) == (1 << s) {
                        buf[read_idx] = 0xFF;
                    }

                    row_bits_read += 1;
                    read_idx += 1;

                    if row_bits_read == self.width {
                        row_bits_read = 0;
                        break 'bytes;
                    }

                    if s == 0 {
                        break 'bits;
                    }
                    s -= 1;
                }
            }
        }
        Ok(())
    }
}
