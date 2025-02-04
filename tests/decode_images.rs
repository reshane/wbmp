use std::fs::File;
use std::io::{BufReader, Cursor};
use std::path::PathBuf;

use wbmp::Decoder;

const TEST_IMAGE_PREFIX: &str = "./tests/images/";

#[test]
fn test_decode_image() {
    const EXPECTED_WIDTH: u32 = 640;
    const EXPECTED_HEIGHT: u32 = 426;
    const EXPECTED_DATA_LEN: usize = 272640;
    let image_file_name = "sample_640x426.wbmp";
    let path = PathBuf::from(TEST_IMAGE_PREFIX).join(image_file_name);
    let file = BufReader::new(File::open(path).expect("Where is the test file?"));
    let mut decoder = Decoder::new(file).expect("Decoder should be instantiable");
    let (width, height) = decoder.dimensions();
    assert!(width == EXPECTED_WIDTH, "Decoded image width should be 640");
    assert!(
        height == EXPECTED_HEIGHT,
        "Decoded image height should be 426"
    );
    let mut img_data = [0_u8; EXPECTED_DATA_LEN];
    decoder
        .read_image_data(&mut img_data)
        .expect("Decoder should be able to read image bytes");
}

#[test]
fn test_decode_4x4() {
    const EXPECTED_WIDTH: u32 = 2;
    const EXPECTED_HEIGHT: u32 = 2;
    const EXPECTED_DATA_LEN: usize = 4;
    let image_bytes = [0x00, 0x00, 0x02, 0x02, 0xC0, 0xC0];

    let reader = BufReader::new(Cursor::new(&image_bytes));
    let mut decoder = Decoder::new(reader).expect("Decoder should be instantiable");
    let (width, height) = decoder.dimensions();
    assert!(width == EXPECTED_WIDTH, "Decoded image width should be 2");
    assert!(
        height == EXPECTED_HEIGHT,
        "Decoded image height should be 2"
    );

    let mut decoded_bytes = [0_u8; EXPECTED_DATA_LEN];
    decoder
        .read_image_data(&mut decoded_bytes)
        .expect("Decoder should be able to read image bytes");
    for byte in decoded_bytes.iter() {
        assert!(*byte == 0xFF);
    }
}

#[test]
fn test_decode_9x9() {
    const EXPECTED_WIDTH: u32 = 9;
    const EXPECTED_HEIGHT: u32 = 9;
    const EXPECTED_DATA_LEN: usize = 81;
    let image_bytes = [
        // headers
        0x00, 0x00, // dimensions
        0x09, 0x09, // image data
        0xFF, 0x80, 0xFF, 0x80, 0xFF, 0x80, 0xFF, 0x80, 0xFF, 0x80, 0xFF, 0x80, 0xFF, 0x80, 0xFF,
        0x80, 0xFF, 0x80,
    ];
    let reader = BufReader::new(Cursor::new(&image_bytes));
    let mut decoder = Decoder::new(reader).expect("Decoder should be instantiable");
    let (width, height) = decoder.dimensions();
    assert!(width == EXPECTED_WIDTH);
    assert!(height == EXPECTED_HEIGHT);
    let mut decoded_bytes = [0_u8; EXPECTED_DATA_LEN];
    decoder
        .read_image_data(&mut decoded_bytes)
        .expect("Decoder should be able to read image bytes");
    for byte in decoded_bytes.iter() {
        assert!(*byte == 0xFF);
    }
}
