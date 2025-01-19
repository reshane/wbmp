
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::PathBuf;

use wbmp::decoder::WbmpDecoder;

const TEST_IMAGE_PREFIX: &str = "./tests/images/";

#[test]
fn test_decode_image() {
    let image_file_name = "sample_640x426.wbmp";
    let path = PathBuf::from(TEST_IMAGE_PREFIX).join(image_file_name);
    let file = BufReader::new(File::open(path).expect("Where is the test file?"));
    let decoder = WbmpDecoder::new(file).expect("Decoder should be instantiable");
    let (width, height) = decoder.dimensions();
    assert!(width == 640, "Decoded image width should be 640");
    assert!(height == 426, "Decoded image height should be 426");
}

#[test]
fn test_decode_4x4() {
    let image_bytes = [
        0x00, 0x00, 0x02, 0x02, 0xC0, 0xC0
    ];
    let reader = BufReader::new(Cursor::new(&image_bytes));
    let mut decoder = WbmpDecoder::new(reader).expect("Decoder should be instantiable");
    let (width, height) = decoder.dimensions();
    assert!(width == 2);
    assert!(height == 2);
    let mut decoded_bytes = [0_u8; 4];
    decoder.read_image_data(&mut decoded_bytes).expect("Decoder should be able to read image bytes");
    println!("{:?}", decoded_bytes);
    for byte in decoded_bytes.iter() {
        println!("{:08b}", byte);
        assert!(*byte == 0xFF);
    }
}

#[test]
fn test_decode_9x9() {
    let image_bytes = [
        // headers
        0x00, 0x00, 
        // dimensions
        0x09, 0x09,
        // image data
        0xFF, 0x80,
        0xFF, 0x80,
        0xFF, 0x80,
        0xFF, 0x80,
        0xFF, 0x80,
        0xFF, 0x80,
        0xFF, 0x80,
        0xFF, 0x80,
        0xFF, 0x80,
    ];
    let reader = BufReader::new(Cursor::new(&image_bytes));
    let mut decoder = WbmpDecoder::new(reader).expect("Decoder should be instantiable");
    let (width, height) = decoder.dimensions();
    assert!(width == 9);
    assert!(height == 9);
    let mut decoded_bytes = [0_u8; 81];
    decoder.read_image_data(&mut decoded_bytes).expect("Decoder should be able to read image bytes");
    println!("{:?}", decoded_bytes);
    for byte in decoded_bytes.iter() {
        println!("{:08b}", byte);
        assert!(*byte == 0xFF);
    }
}
