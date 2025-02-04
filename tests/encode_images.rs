use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use wbmp::ColorType;
use wbmp::Decoder;
use wbmp::Encoder;

const TEST_IMAGE_PREFIX: &str = "./tests/images/";

#[test]
fn test_encode_image() {
    // decode from file
    let image_file_name = "sample_640x426.wbmp";
    let path = PathBuf::from(TEST_IMAGE_PREFIX).join(image_file_name);
    let file = BufReader::new(File::open(path).expect("Where is the test file?"));
    let mut decoder = Decoder::new(file).expect("Decoder should be instantiable");
    let (width, height) = decoder.dimensions();
    assert!(width == 640, "Decoded image width should be 640");
    assert!(height == 426, "Decoded image height should be 426");
    // read into buffer eith rgba8 format
    let mut image_luma8 = vec![0; (width * height) as usize];
    decoder.read_image_data(image_luma8.as_mut_slice()).unwrap();

    // re-encode
    let mut out_bytes = Vec::new();
    let mut encoder = Encoder::new(&mut out_bytes);
    encoder
        .encode(image_luma8.as_slice(), width, height, ColorType::Luma8)
        .unwrap();
    // length
    assert!(out_bytes.len() == 6 + ((640 * 426) / 8));
    // headers
    assert!(out_bytes[0] == 0x00);
    assert!(out_bytes[1] == 0x00);
    // width
    assert!(out_bytes[2] == 0x85);
    assert!(out_bytes[3] == 0x00);
    // height
    assert!(out_bytes[4] == 0x83);
    assert!(out_bytes[5] == 0x2a);
}

#[test]
fn test_encode_2x2_luma8() {
    const WIDTH: u32 = 2;
    const HEIGHT: u32 = 2;
    let img_dat = vec![0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8];
    // rows are 2 long
    // we expect 2 octets, one per row
    // 0b1100_0000

    let mut out_bytes = Vec::new();
    let mut encoder = Encoder::new(&mut out_bytes);
    encoder
        .encode(img_dat.as_slice(), WIDTH, HEIGHT, ColorType::Luma8)
        .expect("Data should be encodeable");
    // headers
    assert!(out_bytes[0] == 0x00);
    assert!(out_bytes[1] == 0x00);
    // width
    assert!(out_bytes[2] == 0x02);
    // height
    assert!(out_bytes[3] == 0x02);
    // image data
    assert!(out_bytes[4] == 0xC0);
    assert!(out_bytes[5] == 0xC0);
}

#[test]
fn test_encode_2x2_rgba8() {
    const WIDTH: u32 = 2;
    const HEIGHT: u32 = 2;
    let img_dat = vec![
        0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8,
        0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8, 0xFF_u8,
    ];
    // rows are 2 long
    // we expect 2 octets, one per row
    // 0b1100_0000

    let mut out_bytes = Vec::new();
    let mut encoder = Encoder::new(&mut out_bytes);
    encoder
        .encode(img_dat.as_slice(), WIDTH, HEIGHT, ColorType::Rgba8)
        .expect("Data should be encodeable");
    // headers
    assert!(out_bytes[0] == 0x00);
    assert!(out_bytes[1] == 0x00);
    // width
    assert!(out_bytes[2] == 0x02);
    // height
    assert!(out_bytes[3] == 0x02);
    // image data
    assert!(out_bytes[4] == 0xC0);
    assert!(out_bytes[5] == 0xC0);
}
